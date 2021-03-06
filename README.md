# Blockchain Research And Development
![badge](https://action-badges.now.sh/NoCtrlZ/blockchain-reseach-and-development?action=test)   
This is research and development for blockchain implementation from scratch.
## Abstract
Creating blockchain proofs the ownership of digital contents with cool way! This blockchain allows all developer to create Dapp with no knowledge about blockchain by just installing wrapper module. This has a revolutionary impact on both creator and blockchain industry. Next age comming soon😎
## Start
```
$ cd integration
$ cargo install --force cargo-make
$ makers run
```
You can access blockchain information on [`http://localhost:3000`](http://localhost:3000) and if you execute same command on same directory, new node will start and be connected to network.  
- Get Blockchain Info  
`/`  
- Get All Nodes In Network  
`/nodes`  
- Get Node Balance  
`/balance`  
- Consensus  
`/consensus`  
- Get Network State  
`/network`  
- Get Blockchain State  
`/blockchain`  
- Create New Block  
`/crate_new_block`  
## Progress
- 🥚 0~30%
- 🐥 31~60%
- 🐓 61~90%
- 🍗 91%~

Component | R&D | Integrate
:------------ | :-------------| :-------------
Proof Of Work|🍗|🍗
Adjustment Difficultty|🥚|🥚
Lamport Signature|🍗|🍗
Merkle Tree|🐓|🥚
P2P Network|🍗|🍗
Wallet|🍗|🍗
Block Structure|🐓|🐓
Transaction Type|🐓|🐥
UTXO|🍗|🍗
## Directory
### 🛰️blockchain
Development about blockchain data structure including struct of `blockchain` itself, `block` and `transaction`. Also has methods to chage the data structure like `create_new_block`, `send_transaction`, `proof_of_work` and so on.
### 📡p2p
Development about p2p client including syncing method with other nodes. Creating Json server which is used as endpoint by user from scratch.
### 🚀integration
Integration result of research and development. This behaves as actual blockchain which proofs the ownership of digital contents. So cool!
### 🛸lamport-sig
Development about lamport signature which has tolerance for quantum computer. Generating private key and signing transaction.
### ☄️wallet
Development about wallet. Wallet struct including private key, public key and address, and methods including sign, verify and genelate.
### 👽transaction
Development about transaction message. Signing transaction with private key, issuing certificate transaction and transfer token transaction, verifying transaction.
### 🛫merkle-tree
Development about merkle tree which is data structure enables us to summarize and verify the transactions. Wallet has summarized transactions and checks whether they are altered.
### 🦅utxo
Development about uxto which is data structure enables us to prevent multiple payment and keep transactions history high secrecy.
### 🔫cli
CLI to test integration blockchain server. This has function to issue certificate and send transaction with private key sign.
## Test
```
$ makers it
```
