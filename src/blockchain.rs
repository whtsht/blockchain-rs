use crate::{block::Block, server::FullChain, transaction::Transaction};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
    nodes: HashSet<String>,
}

impl Blockchain {
    pub fn hash(block: &Block) -> String {
        sha256::digest(serde_json::to_string(block).unwrap())
    }

    pub fn valid_proof(last_proof: u64, proof: u64) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let guess_hash = sha256::digest(guess);

        &guess_hash[0..4] == "0000"
    }

    pub fn new() -> Self {
        let mut blockchain = Self {
            chain: Vec::new(),
            current_transactions: Vec::new(),
            nodes: HashSet::new(),
        };

        blockchain.new_block(100, Some("1".into()));

        blockchain
    }

    pub fn chain(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn new_block(&mut self, proof: u64, previous_hash: Option<String>) -> Block {
        let block = if let Some(previous_hash) = previous_hash {
            Block::new(
                self.chain.len() as u32 + 1,
                self.current_transactions.clone(),
                proof,
                previous_hash,
            )
        } else {
            Block::new(
                self.chain.len() as u32 + 1,
                self.current_transactions.clone(),
                proof,
                Self::hash(self.chain.last().unwrap()),
            )
        };
        self.current_transactions.clear();
        self.chain.push(block.clone());
        block
    }

    pub fn new_transaction(&mut self, transaction: Transaction) -> usize {
        self.current_transactions.push(transaction);
        self.chain.len() + 1
    }

    pub fn proof_of_work(&mut self, last_proof: u64) -> u64 {
        let mut proof = 0;

        while !Self::valid_proof(last_proof, proof) {
            proof += 1;
        }

        proof
    }

    pub fn register_node(&mut self, address: String) {
        self.nodes.insert(address);
    }

    pub fn valid_chain(&self, chain: &Vec<Block>) -> bool {
        let mut last_block = self.last_block();
        let mut current_index = 1;
        while current_index < chain.len() {
            let block = &chain[current_index];
            if block.previous_hash != Self::hash(last_block) {
                return false;
            }

            if !Self::valid_proof(last_block.proof, block.proof) {
                return false;
            }
            last_block = block;
            current_index += 1;
        }

        true
    }

    pub async fn resolve_conflicts(&mut self) -> bool {
        let mut max_length = self.chain.len();
        let mut new_chain: Option<Vec<Block>> = None;
        for node in self.nodes.iter() {
            let text = reqwest::get(format!("http://{}/chain", node))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let fullchain: FullChain = serde_json::from_str(&text).unwrap();
            if fullchain.length > max_length && self.valid_chain(&fullchain.chain) {
                max_length = fullchain.length;
                new_chain = Some(fullchain.chain);
            }
        }
        if let Some(new_chain) = new_chain {
            self.chain = new_chain;
            true
        } else {
            false
        }
    }
}
