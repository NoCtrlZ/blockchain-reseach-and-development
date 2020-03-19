struct Tree {
    leaves: Vec<String>,
    layer: Vec<String>,
    root: String
}

#[derive(Debug)]
struct Transactions {
    transactions: Vec<Transaction>
}

#[derive(Debug)]
struct Transaction {
    amount: u64,
    sender: String,
    recipient: String
}

impl Tree {
    fn new(transactions: Vec<String>) -> Tree {
        Tree {
            leaves: transactions.clone(),
            layer: transactions.clone(),
            root: "".to_string()
        }
    }
}

impl Transactions {
    fn new() -> Transactions {
        Transactions {
            transactions: Vec::new()
        }
    }

    fn send_transaction(&mut self, amount: u64, sender: &str, recipient: &str) {
        self.transactions.push(Transaction {
            amount: amount,
            sender: sender.to_string(),
            recipient: recipient.to_string()
        })
    }
}

fn main() {
    let mut transactions = Transactions::new();
    send_transactions(&mut transactions);
    println!("{:?}", transactions);
}

fn send_transactions(transactions: &mut Transactions) {
    transactions.send_transaction(100, "alice", "bob");
}