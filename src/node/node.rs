use crate::block::*;
use crate::transaction::*;
use crate::node::mempool::*;

use std::collections::HashSet;

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub blockchain: Vec<Block>,
    pub mempool: Mempool,
    pub peers: HashSet<String>,
    pub difficulty: u64,
}

//TODO: Implement Node logic
impl Node {
    //initialize
}