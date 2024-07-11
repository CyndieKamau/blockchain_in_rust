use sha2::{sha256, Digest};


pub struct MerkleNode {
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
    data: String,
}

impl MerkleNode {
    pub fn new_merkle_tree(data: &str) -> Self {

        MerkleNode {
            left: None,
            right: None,
            data: String::from(data),
        }

    }

    pub fn combine_left_right_node(left: MerkleNode, right: MerkleNode) -> Self {

        let combined_data = format!("{} {}", left.data, right.data);
        let mut hasher = sha256::new();
        hasher.update(combined_data.as_bytes());
        let new_data = format!("{:x}", hasher.finalize()); 

        MerkleNode {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            data: new_data,
        }

    }

    pub fn new_leaf(data: &str) -> Self {

        let mut hasher = sha256::new();
        hasher.update(data.as_bytes());
        let new_data = format!("{:x}", hasher.finalize());

        MerkleNode {
            left: None,
            right: None,
            data: new_data,
        }
    }
}


#[derive(Debug)]
pub struct MerkleTree {
    root: Option<MerkleNode>
}

impl MerkleTree {
    pub fn new(transactions: Vec<String>) -> Self {
        let mut nodes = Vec::new();

        for transaction in transactions {
            nodes.push(MerkleNode::leaf(&transaction));
        }

        while nodes.len() > 1 {
            let mut new_level = Vec::new();

            while let Some(left_node) = nodes.pop() {
                if let Some(right_node) = nodes.pop() {
                    new_level.push(MerkleNode::combine(&left_node, &right_node));
                } else {
                    new_level.push(MerkleNode::combine(&left_node, &left_node));
                }
            }

            nodes = new_level;
        }

        MerkleTree {
            root: nodes.pop(),
        }
    }

    pub fn root(&self) -> Option<String> {
        self.root.as_ref().map(|node| node.data.clone())
    }    
}