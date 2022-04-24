# Distributor

A simple reward distribution contract. It's using the standard implementation of cw20-base
from cw-plus and my own stacking/reward contract.

## Testnet deployment
The contracts deployed to the testnet are:
- wtoken: terra1mjddpgxshxey00ldmyu0uzhvcggs6484syxerg
- staking: terra1d4xnyjujrxn4lwyqrhj8k7v792h0nqevm5zfla

## How to deploy
All the deployment scripts are in the `deploy` folder.
To deploy, you will need a resonably recent version of python 3.

To install the dependencies use pipenv:
```sh
cd deploy
pipenv install -r requirements.txt 
```

In order to deploy you will need to :
1. create a secret.key file 
2. enter the memo of a wallet you want to use to deploy
3. run the command

The secret key file is excluded from github. There is one config file per deployment environment (ex: testnet).

The deployment command is 
```sh
cd deploy
pipenv run python3 deploy.py --config testnet.conf
```

## How to use the tests scripts

You will need a resonably recent version of python (I have tested with Python 3.8.9).

To install the dependencies use pipenv:
```sh
cd tests
pipenv install -r requirements.txt
```

In order to run tests you will need to :
1. create a secret.key file containing the memomnic (note: you can have multiple secret files)
2. enter the memo of a wallet you want to use to deploy
3. run the command

The secret key file is excluded from github. 

The test command could look like this:
```sh
pipenv shell
# stake token
python3 deposit.py --reward_contract $REWARD --cw20_contract $TOKEN --amount 2000 --secret secret_1.key
python3 deposit.py --reward_contract $REWARD --cw20_contract $TOKEN --amount 1000 --secret secret_2.key

# send reward
python3 reward.py --reward_contract $REWARD --balance 1000uusd

# deposit more
python3 deposit.py --reward_contract $REWARD --cw20_contract $TOKEN --amount 1000 --secret secret_3.key

# send reward
python3 reward.py --reward_contract $REWARD --balance 1000uusd

# Claim accrued reward
python3 claim.py --reward_contract $REWARD --secret secret_1.key
python3 claim.py --reward_contract $REWARD --secret secret_2.key
python3 claim.py --reward_contract $REWARD --secret secret_3.key

# unstake token
python3 withdraw.py --reward_contract $REWARD --amount 1000 --secret secret_1.key
python3 withdraw.py --reward_contract $REWARD --amount 1000 --secret secret_1.key
python3 withdraw.py --reward_contract $REWARD --amount 1000 --secret secret_2.key
python3 withdraw.py --reward_contract $REWARD --amount 1000 --secret secret_3.key
```

## Scripts function

The script can used as follow:
- deposit.py: deposit wtoken in the reward contract
- withdraw.py: withdraw deposited wtoken from the reward contract
- claim.py: claim the accrued earnings
- reward.py: send rewards to the contract
