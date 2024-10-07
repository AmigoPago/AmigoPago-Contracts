# AmigoPago-Contracts

The AmigoPago platform will utilize smart contracts on the Stellar blockchain to handle various aspects of the system. These smart contracts will be developed using Soroban, Stellar's smart contract platform. Here's an overview of the key smart contracts:

1. AMI Token Management Contract: --CAN33VWQO6CBG7CI6WHUYMAYT5UYKHCRJH6MAFP5JYRQGX474SMBV3RL--
This contract will manage AMI token issuance, distribution, and burning. It will include functions for:
- Minting new AMI tokens based on user activity and cashback rewards
- Transferring AMI tokens between user wallets
- Burning AMI tokens when used for purchases or as collateral

2. Remittance Contract: --CCU5Z4ASKZK2FK7QOG7UMNNHJTATFCNGJRZ4WZDDI6IF7CQ37VMEOQG2--
This contract will handle the core remittance functionality, including:
- Initiating cross-border transfers using USDC
- Calculating and applying transaction fees
- Triggering AMI token rewards for completed transfers

3. Credit System Contract: --CDBXJG5XHFF3K7EMCXAYONWIWU3SK5K3TT6JEA5KSMECSWVJI4QFMOJU--
This contract will manage the USDC-backed credit system, with functions for:
- Locking AMI tokens as collateral for micro-loans
- Calculating credit limits based on collateral value
- Managing loan repayments and releasing collateral

* USDC MOCKUP CONTRACT: --CC2GOBGNJQBNX7QY2WPIFF7QAYPHY56UDVDDEHY3EQ6YAJPQVW2U77Z4--

These smart contracts will interact with each other and the broader AmigoPago system to ensure secure, transparent, and efficient operations on the Stellar blockchain. They will be thoroughly tested and audited to ensure security and compliance with relevant regulations.