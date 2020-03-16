# Blockchain Research And Development
![badge](https://action-badges.now.sh/NoCtrlZ/blockchain-reseach-and-development?action=test)   
This is research and development for blockchain implementation from scratch.
## Setup
### Blockchain
```
$ cd blockchain && cargo run
```
## Directory
### blockchain
Development about blockchain data structure including struct of `blockchain` itself, `block` and `transaction`. Also has methods to chage the data structure like `create_new_block`, `send transaction`, `proof_of_work` and so on.
### p2p
Development about p2p client including syncing method with other nodes.
### integration
Development about integration blockchain and p2p.
