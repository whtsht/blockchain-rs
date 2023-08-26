use crate::{block::Block, blockchain::Blockchain, transaction::Transaction};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{io::Result, sync::Mutex};

struct ServerState {
    blockchain: Mutex<Blockchain>,
}

pub async fn main(ip_addr: &str, port: u16) -> Result<()> {
    let state = web::Data::new(ServerState {
        blockchain: Mutex::new(Blockchain::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(mine)
            .service(new_transaction)
            .service(full_chain)
            .service(register_nodes)
            .service(consensus)
    })
    .bind((ip_addr, port))?
    .run()
    .await
}

static NODE_IDENTIFIER: Lazy<String> = Lazy::new(|| uuid::Uuid::new_v4().to_string());

#[get("/mine")]
async fn mine(state: web::Data<ServerState>) -> impl Responder {
    let mut blockchain = state.blockchain.lock().unwrap();
    let last_proof = blockchain.last_block().proof;
    let proof = blockchain.proof_of_work(last_proof);
    blockchain.new_transaction(Transaction::new("0".into(), NODE_IDENTIFIER.clone(), 1));
    let last_block = blockchain.last_block();
    let previous_hash = Blockchain::hash(last_block);
    let block = blockchain.new_block(proof, Some(previous_hash));

    web::Json(block)
}

#[post("/transactions/new")]
async fn new_transaction(
    transaction: web::Json<Transaction>,
    state: web::Data<ServerState>,
) -> impl Responder {
    let index = state
        .blockchain
        .lock()
        .unwrap()
        .new_transaction(transaction.into_inner());
    HttpResponse::Ok().body(format!("Transaction will be added to Block {}", index))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FullChain {
    pub chain: Vec<Block>,
    pub length: usize,
}

#[get("/chain")]
async fn full_chain(state: web::Data<ServerState>) -> impl Responder {
    let chain = state.blockchain.lock().unwrap().chain().clone();
    let length = chain.len();
    let full_chain = FullChain { chain, length };
    web::Json(full_chain)
}

#[post("/nodes/register")]
async fn register_nodes(
    nodes: web::Json<Vec<String>>,
    state: web::Data<ServerState>,
) -> impl Responder {
    let mut blockchain = state.blockchain.lock().unwrap();
    for node in nodes.into_inner() {
        blockchain.register_node(node);
    }
    HttpResponse::Ok().body(format!("New nodes have been added"))
}

#[get("/nodes/resolve")]
async fn consensus(state: web::Data<ServerState>) -> impl Responder {
    let mut blockchain = state.blockchain.lock().unwrap();
    let replaced = blockchain.resolve_conflicts().await;

    if replaced {
        HttpResponse::Ok().body(format!("Our chain was replaced"))
    } else {
        HttpResponse::Ok().body(format!("Our chain is authoritative"))
    }
}
