//Block tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_block_header() {
        let block = BlockHeader {
            index: 0,
            timestamp: 0,
            block_hash: String::from("0"),
            prev_hash: String::from("0"),
            nonce: 0,
            difficulty: 3,

        };

        assert_eq!(block.index, 0);
        assert_eq!(block.timestamp, 0);
        assert_eq!(block.block_hash, 0);
        assert_eq!(block.prev_hash, 0);
        assert_eq!(block.nonce, 0);
        assert_eq!(block.difficulty, 0);

    }

    #[test]
    fn test_create_new_block() {
        let header = BlockHeader {
            index: 0,
            timestamp: 0,
            block_hash: String::from("0"),
            prev_hash: String::from("0"),
            nonce: 0,
            difficulty:0
        };

        let tx1 = Transaction {
            sender: String::from("Alice"),
            receiver: String::from("Bob"),
            amount: 20,
            timestamp: 0,
            signature: String::from("0"),
        };

        let tx2 = Transaction {
            sender: String::from("Uhn"),
            receiver: String::from("May"),
            amount: 10,
            timestamp: 0,
            signature: String::from("0"),
        };

        let block = Block {
            header: header,
            transactions: vec![tx1, tx2],
        };

        assert_eq!(block.header.timestamp, 0);
        assert_eq!(block.transactions.len(), 2);


    }
}