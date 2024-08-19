//Account struct will have the following fields:
//Account, balance, private key, public key

use crate::Transaction::*;

pub struct Account {
    pub account: String,
    pub balance: f64,
    pub private_key: String,
    pub public_key: String,
}

impl Account {
    pub fn new_account(account: String, balance: f64, private_key: String, public_key: String) -> Self {
        Account {
            account,
            balance,
            private_key,
            public_key
            
        }
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn send_transaction(&mut self, receiver: String, amount: f64, signature: String) -> Result<Transaction, &'static str> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(Transaction {
                sender: self.address.clone(),
                receiver,
                amount,
                timestamp, //get the timestamp when tx happened
                signature,
            })
        } else {
            Err("Insufficient balance")
        }
    }

    pub fn receive_transaction(&mut self, transaction: &Transaction) {
        if transaction.receiver == self.address {
            self.balance += transaction.amount;
        }
    }
}