# Distributor

A simple reward distributionb contract. It's using the standard implementation of cw20-base
from cw-plus and my own stacking/reward contract



## How to deploy
All the deployment scripts a in the `deploy` folder.
To deploy, you will need a resonably recent version of python 3.

To install the dependencies use pip:
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

To install the dependencies use pip:
```sh
cd tests
pipenv install -r requirements.txt
```

In order to run tests you will need to :
1. create a secret.key file
2. enter the memo of a wallet you want to use to deploy
3. run the command

The secret key file is excluded from github. 

The test command should look like this:
```sh
pipenv shell
python3 deposit.py --reward_contract $REWARD --cw20_contract $TOKEN --amount 1000
python3 withdraw.py --reward_contract $REWARD --amount 1000
```

## Scripts function

The script can used as follow:
- deposit.py: deposit wtoken in the reward contract
- withdraw.py: withdraw deposited wtoken from the reward contract