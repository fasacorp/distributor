import argparse
from datetime import datetime
import logging
import time

from terra_sdk.client.lcd import LCDClient
from terra_sdk.core.wasm import MsgExecuteContract
from terra_sdk.key.mnemonic import MnemonicKey
from terra_sdk.client.lcd.api.tx import CreateTxOptions

CHAIN_ID = "bombay-12"
LCD_URL = "https://bombay-lcd.terra.dev"
MULTIPLIER = 1000000

def get_client_wallet(secret):
    '''Create the client & wallet necessary for the deployment'''
    # create the lcd
    memo = secret
    logging.info("Creating client with for %s using %s", CHAIN_ID, LCD_URL)
    lcd = LCDClient(chain_id=CHAIN_ID, url=LCD_URL)    
    with open(memo, 'r') as memo_file:
        memo_phrase = memo_file.readline().strip()
        wallet = lcd.wallet(MnemonicKey(memo_phrase))  
        return lcd, wallet

def claim(lcd, wallet, reward_contract, gas_price):
    # Generate reward deposit message
    execution_msg = {
        "claim": {}
    }
    msg = MsgExecuteContract(
        wallet.key.acc_address,
        reward_contract,
        execution_msg,        
    )       
    logging.info("Claiming rewards")
    execute_tx = wallet.create_and_sign_tx(
        CreateTxOptions(msgs=[msg], gas_prices=gas_price, gas_adjustment="2"))
    execute_tx_result = lcd.tx.broadcast(execute_tx)   
    if execute_tx_result.is_tx_error():
        logging.warning("Claiming failed!")
        logging.warning("%s", execute_tx_result)
        raise Exception("Claim failed")
    else:
        logging.info("Claim successful")

def main(reward_contract, gas_price, secret):    
    # build lcd and wallet objects
    lcd, wallet = get_client_wallet(secret)
    claim(lcd, wallet, reward_contract, gas_price)

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Run deposit tests on the testnet')
    parser.add_argument('--reward_contract', type=str, help='The address of the reward contract', required=True)    
    parser.add_argument('--gas_price', type=str, help='The gas price to use', required=False, default='0.15uusd')    
    parser.add_argument('--secret', type=str, help='The secret key to use', required=False, default='secret_1.key')    

    args = parser.parse_args()
    logging.basicConfig(format='%(asctime)s %(message)s', level=logging.INFO)
    main(args.reward_contract, args.gas_price, args.secret)
