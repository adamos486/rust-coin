use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UtxoRef {
    txid: [u8; 32],
    vout: u32,
}

#[derive(Debug)]
pub struct TxInput {
    utxo_ref: UtxoRef,
    signature: [u8; 64],
}

#[derive(Debug)]
pub struct TxOutput {
    address: [u8; 20],
    amount: u64,
}

#[derive(Debug)]
pub struct Transaction {
    version: u32,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    locktime: u32,
}