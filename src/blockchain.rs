use serde_derive::Serialize;
use sha2::{ Sha256, Digest};
use std::fmt::Write;


#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sedner: String,
    reciever: String,
    amount: f32
}