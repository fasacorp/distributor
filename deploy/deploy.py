import argparse
import base64
import configparser
import json
import logging
import os
import posixpath
from sre_constants import SUCCESS
from time import sleep
import traceback

from terra_sdk.client.lcd import LCDClient
from terra_sdk.core.coins import Coins
from terra_sdk.core.wasm import (MsgExecuteContract, MsgInstantiateContract,
                                 MsgStoreCode)
from terra_sdk.key.mnemonic import MnemonicKey
from terra_sdk.client.lcd.api.tx import CreateTxOptions


def load_config(config_file):
    ''' Load the config file for the deployment'''
    parser = configparser.ConfigParser()
    parser.read(config_file)

    return parser

def broadcast(lcd, tx):
    ''' broadcast a tx, will retry 3 times.'''
    failure = 0    
    while failure < 3: 
        try:                
            tx_result = lcd.tx.broadcast(tx)
            return tx_result
        except:
            logging.info("Failed, trying again")            
            failure += 1    
            traceback.print_exc()
            sleep(30)

    if failure>3:
        raise Exception("failed to broadcast")

def get_client_wallet(conf):
    '''Create the client & wallet necessary for the deployment'''
    # create the lcd
    chain_id = conf['client']['chainid']
    lcd_url = conf['client']['lcd']
    memo = conf['client']['mnemonic']
    logging.info("Creating client with for %s using %s", chain_id, lcd_url)
    lcd = LCDClient(chain_id=chain_id, url=lcd_url)    
    with open(memo, 'r') as memo_file:
        memo_phrase = memo_file.readline()
        memo_phrase = memo_phrase.strip()
        wallet = lcd.wallet(MnemonicKey(memo_phrase))  
        return lcd, wallet

def get_artifacts(conf):
    folder = conf['deployment']['artifacts_folder']
    res = {}
    for x in os.listdir(folder):
        if x.endswith(".wasm"):
            res[x] = posixpath.join(folder,x)
    return res

def upload_code(conf, lcd, wallet, sequence):
    folder = conf['deployment']['artifacts_folder']
    contracts = conf['deployment']['contracts'].split(',')
    gas_price = conf['deployment']['gas_price']
    res = {}
    for contract in contracts:
        artifact = conf[contract]["artifact"]
        path = posixpath.join(folder,artifact)
        with open(path, "rb") as contract_file:
            file_bytes = base64.b64encode(contract_file.read()).decode()
            store_code = MsgStoreCode(wallet.key.acc_address, file_bytes)
            store_code_tx = wallet.create_and_sign_tx(
                CreateTxOptions(
                    msgs=[store_code],
                    gas_prices=gas_price,
                    gas_adjustment="1.7",
                    sequence=sequence,
                ))
            store_code_tx_result = broadcast(lcd, store_code_tx)
            code_id = store_code_tx_result.logs[0].events_by_type["store_code"]["code_id"][0]
            logging.info("Deployed contract %s with codeid %s", contract, code_id)
            res[contract] = code_id
            sequence += 1 
    return res, sequence

def replace_template(msg, sender, addresses):    
    if "@sender" in msg:
        msg = msg.replace("@sender", sender)
    for key, addr in addresses.items():        
        addr_key = key + "@addr"
        if addr_key in msg:
            msg = msg.replace(addr_key, addr)
    return msg

def instanciate_contract(conf, code_id, addresses, lcd, wallet, contract, gas_price, sequence):
    inst_msg = conf[contract]['instanciate_msg']      
    admin    = conf[contract].get('admin') or wallet.key.acc_address
    inst_msg = replace_template(inst_msg, wallet.key.acc_address, addresses)
    json_msg = json.loads(inst_msg)

    logging.info("Instanciating %s with message: \n%s", contract, json_msg)
    logging.info("Admin is %s", admin)
    instantiate = MsgInstantiateContract(
        sender = wallet.key.acc_address,
        admin = admin,
        code_id = code_id,
        init_msg = json_msg,
    )
    instantiate_tx = wallet.create_and_sign_tx(
        CreateTxOptions(
            msgs=[instantiate], 
            gas_prices=gas_price, 
            gas_adjustment="1.7", 
            sequence=sequence)
    )
    instantiate_tx_result = broadcast(lcd, instantiate_tx)
    if instantiate_tx_result.is_tx_error():
        logging.warning("Transaction failed\n%s", instantiate_tx_result)
    else:
        contract_address = instantiate_tx_result.logs[0].events_by_type["instantiate_contract"]["contract_address"][0]
        logging.info("Contract address for %s: %s", contract, contract_address)
        return contract_address

def instanciate_contracts(conf, code_ids, lcd, wallet, sequence):
    logging.info("Instanciating CAPSULE contracts")
    gas_price = conf['deployment']['gas_price']
    contracts = conf['deployment']['contracts'].split(',')
    addresses = {}
    
    for contract in contracts:
        code_id  = code_ids[contract]
        if 'instanciate_many' in conf[contract]:
            inst_msgs = conf[contract]['instanciate_many'].split(',')  
            for sub_inst in inst_msgs:
                addresses[sub_inst] = instanciate_contract(conf, code_id, addresses, lcd, wallet, sub_inst, gas_price, sequence) 
                sequence += 1 
        else:
            addresses[contract] = instanciate_contract(conf, code_id, addresses, lcd, wallet, contract, gas_price, sequence)
            sequence += 1 

    return addresses, sequence

def process_execution_msg(conf, deployed_addr, lcd, wallet, sequence):
    contracts = conf['deployment']['contracts'].split(',')

    gas_price = conf['deployment']['gas_price']
    coins = None

    for contract in contracts:
        if 'execution_msg' in conf[contract]:
            contract_address = deployed_addr[contract]
            section = conf[contract]['execution_msg'] 
            for key, exec in conf.items(section):
                execution_array = replace_template(exec, wallet.key.acc_address, deployed_addr)
                execution_array = execution_array.split("|")
                if (len(execution_array) > 0):
                    execution = execution_array[0]

                    if (len(execution_array) > 1):
                        coins = Coins.from_str(execution_array[1].strip())

                    logging.info( "Executing %s = %s", key, execution)
                    msg = MsgExecuteContract(
                        wallet.key.acc_address,
                        contract_address,
                        json.loads(execution),
                        coins,
                    )
                    execute_tx = wallet.create_and_sign_tx(
                        CreateTxOptions(
                            msgs=[msg], 
                            gas_prices=gas_price, 
                            gas_adjustment="1.7", 
                            sequence=sequence)
                    )
                    execute_tx_result = broadcast(lcd, execute_tx)
                    if execute_tx_result.is_tx_error():
                        logging.warning("Execution Err")
                        logging.warning("tx result = %s", execute_tx_result)
                        raise Exception("Execution error")
                    else:
                        logging.info("Execution OK")
                    sequence += 1 
                    
def main(conf_file):
    try:
        logging.info("using config file '%s'", conf_file)    
        conf = load_config(conf_file)
        # instanciate the client en wallet to deploy
        lcd, wallet = get_client_wallet(conf)

        sequence = lcd.auth.account_info(wallet.key.acc_address).sequence
        
        # deploy the compiled code
        code_ids, sequence = upload_code(conf, lcd, wallet, sequence)

        # instanciate the contracts
        deployed_addr, sequence = instanciate_contracts(conf, code_ids, lcd, wallet, sequence)

        process_execution_msg(conf, deployed_addr, lcd, wallet, sequence)

        logging.info("Artifacts were deployed as follow:")
        for artifact, addr in deployed_addr.items():
            logging.info("- %s: %s", artifact, addr)
    except Exception as e:
        logging.exception(e)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Deploy all the contracts to the testnet')
    parser.add_argument('--config', type=str, help='The deployment config file', required=True)
    

    args = parser.parse_args()
    logging.basicConfig(format='%(asctime)s.%(msecs)03d %(levelname)-8s [%(filename)s:%(lineno)d] %(message)s', level=logging.INFO)

    main(args.config)
