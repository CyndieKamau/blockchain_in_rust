pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub timestamp: u64,
    pub signature: String,
}

impl Transaction {
    pub fn new_transaction(sender: String, receiver: String, amount: u64, timestamp: u64, signature: String) -> Self {

        Transaction {
            sender,
            receiver,
            amount,
            timestamp,
            signature
        }
    }
}