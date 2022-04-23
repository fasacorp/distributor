# Distributor

A simple reward distributionb contract. It's using the standard implementation of cw20-base
from cw-plus and my own stacking/reward contract



## How to deploy
All the deployment scripts a in the `deploy` folder.
To deploy, you will need a resonably recent version of python 3.

To install the dependencies use pip:
```
cd deploy
pipenv install -r requirements.txt 
```

In order to deploy you will need to :
1. create a secret.key file
2. enter the memo of a wallet you want to use to deploy
3. run the command

The secret key file is excluded from github. There is one config file per deployment environment (ex: testnet).

The deployment command is 
```
cd deploy
pipenv run python3 deploy.py --config testnet.conf
```

## How to use the test scripts

You will need a resonably recent version of python (I have tested with Python 3.8.9).

To install the dependencies use pip:
```
pipenv install -r requirements.txt
```

In order to run tests you will need to :
1. create a secret.key file
2. enter the memo of a wallet you want to use to deploy
3. run the command

The secret key file is excluded from github. 

The test command should look like this:
```
python3 deposit.py 
```

Vault is the main contract vault and strategy is the name of the strategy to deposit to.
