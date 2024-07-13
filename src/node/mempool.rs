use crate::transaction::*;

pub struct Mempool {
    pub transactions: Vec<Transaction>,
}

impl Mempool {
    //initialize the mempool
    pub fn new() -> Mempool {

        Mempool {
            transactions: Vec::new(),
        }
    }

    //add a tx to the mempool
    pub fn add_new_transaction(&mut self, tx: Transaction) -> Result<(), &'static str>{
        self.transactions.push(tx);

        Ok(())
    }

    //remove a tx from the mempool
    pub fn remove_transaction(&mut self, tx: Transaction) -> Result<(), &'static str>{
        if let Some(index) = self.transactions.iter().position(|tx| tx == transaction) {
            self.transactions.remove(index);
            Ok(())
        } else {
            Err("Transaction not found")
        }
    }

    //retrieve all txs to a certain limit
    pub fn get_transactions(&self, limit: usize) -> Vec<Transaction> {
        self.transactions.iter().take(limit).cloned().collect()
    }

    //clear mempool
    pub fn clear_mempool(&mut self) {
        self.transactions.clear();
    }

    //get no of txs in mempool
    pub fn get_transactions_no(&self) -> usize {
        self.transactions.len()
    }
}