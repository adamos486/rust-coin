use std::collections::HashMap;



pub struct UtxoDatabase {
    utxos: HashMap<UtxoRef, TxOutput>,
}

pub impl UtxoDatabase {
    pub fn new() -> Self {
        UtxoDatabase {
            utxos: HashMap::new(),
        }
    }

    pub fn add_utxo(&mut self, utxo_ref: UtxoRef, output: TxOutput) {
        self.utxos.insert(utxo_ref, output);
    }

    pub fn get_utxo(&self, utxo_ref: &UtxoRef) -> Option<&TxOutput> {
        self.utxos.get(utxo_ref)
    }

    pub fn remove_utxo(&mut self, utxo_ref: &UtxoRef) -> Option<TxOutput> {
        self.utxos.remove(utxo_ref)
    }

    pub fn get_all_utxos(&self) -> &HashMap<UtxoRef, TxOutput> {
        &self.utxos
    }
}

