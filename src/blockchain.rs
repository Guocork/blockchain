use serde_derive::Serialize;
use sha2::{ Sha256, Digest};
use std::fmt::Write;


#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    reciever: String,
    amount: f32
}

#[derive(Serialize, Debug)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transaction: Vec<Transaction>
}

pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>, // 当前未被打包进区块的交易
    difficulty: u32,          // 挖矿的难度
    miner_addr: String,       // 矿工的地址
    reward: f32               // 矿工所获得的奖励数量
}


impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0
        };

        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, reciever: String, amount: f32) -> bool {
        self.curr_trans.push(Transaction {
            sender,
            reciever,
            amount
        });

        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap()
        };

        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: self.last_hash(),
            difficulty: self.difficulty
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            reciever: self.miner_addr.clone(),
            amount: self.reward
        };

        let mut block = Block {
            header,
            count: 0,
            transaction: vec![]
        };

        block.transaction.push(reward_trans);
        block.transaction.append(&mut self.curr_trans);
        block.count = block.transaction.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transaction.clone());
        Chain::proof_of_work(&mut block.header);

        println!("{:?}", &block);
        self.chain.push(block);
        true
    }
}