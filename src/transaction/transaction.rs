use std::collections::HashMap;

use crate::transaction::database::UtxoDatabase;

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

pub struct TransactionProcessor {
    utxo_db: UtxoDatabase,
    mempool: Vec<Transaction>,
}

impl TransactionProcessor {
    pub fn new() -> Self {
        TransactionProcessor {
            utxo_db: UtxoDatabase::new(),
            mempool: Vec::new(),
        }
    }

    pub fn validate_and_add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        let mut total_input_amount = 0;
        for input in &transaction.inputs {
            if let Some(output) = self.utxo_db.get_utxo(&input.utxo_ref) {
                total_input_amount += output.amount;
            } else {
                return Err(format!("Input references non-existent UTXO: {:?}", input.utxo_ref));
            }
        }

        let total_output_amount: u64 = transaction.outputs.iter()
            .map(|output| output.amount)
            .sum();

        if total_input_amount < total_output_amount {
            return Err(format!(
                "Insufficient funds: inputs={}, outputs={}",
                total_input_amount, total_output_amount
            ));
        }

        for input in &transaction.inputs {
            self.utxo_db.remove_utxo(&input.utxo_ref);
        }

        let txid = generate_txid(&transaction);
        for (vout, output) in transaction.outputs.iter().enumerate() {
            let utxo_ref = UtxoRef {
                txid,
                vout: vout as u32,
            };
            self.utxo_db.add_utxo(utxo_ref, output.clone());
        }

        self.mempool.push(transaction);
        Ok(())
    }

    pub fn get_transaction_by_input<'a>(&'a self, utxo_ref: &UtxoRef) -> Option<&'a Transaction> {
        self.mempool.iter().find(|tx| {
            tx.inputs.iter().any(|input| input.utxo_ref == *utxo_ref)
        })
    }

    pub fn get_total_supply(&self) -> u64 {
        self.utxo_db.get_all_utxos().values()
            .map(|output| output.amount)
            .sum()
    }
}

struct Wallet {
    address: [u8; 20],
}

impl Wallet {
    pub fn new(address: [u8; 20]) -> Self {
        Wallet { address }
    }

    pub fn create_transaction(
        &self,
        utxo_db: &UtxoDatabase,
        recipient: [u8; 20],
        amount: u64,
    ) -> Result<Transaction, String> {
        let our_utxos: Vec<(UtxoRef, &TxOutput)> = utxo_db.get_all_utxos()
            .iter()
            .filter(|(_, output)| {
                output.address == self.address
            })
            .map(|(utxo_ref, output)| {
                (*utxo_ref, output)
            })
            .collect();

        let total_available: u64 = our_utxos.iter()
            .map(|(_, output)| {
                output.amount
            })
            .sum();

        if total_available < amount {
            return Err(format!(
                "Insufficient funds: available={}, required={}",
                total_available, amount
            ));
        }

        let mut selected_utxos = Vec::new();
        let mut selected_amount = 0;

        for (utxo_ref, output) in our_utxos {
            selected_utxos.push(utxo_ref);
            selected_amount += output.amount;
            if selected_amount >= amount {
                break;
            }
        }

        let inputs: Vec<TxInput> = selected_utxos
            .into_iter()
            .map(|utxo_ref| {
                let signature = [0u8; 64];
                TxInput { utxo_ref, signature }
            })
            .collect();

        let mut outputs = vec![
            TxOutput { address: recipient, amount }
        ];
    }
}