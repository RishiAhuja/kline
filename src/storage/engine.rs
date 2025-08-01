use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

use base64::{engine::general_purpose, Engine as _};

pub struct Kline {
    store: HashMap<Vec<u8>, Vec<u8>>,
    log: File,
}

impl Kline {
    pub fn open(path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(path)?;

        let mut store = HashMap::new();
        let reader = BufReader::new(&file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.splitn(3, ' ').collect();

             match parts.as_slice() {
                ["put", key_b64, value_b64] => {
                    if let (Ok(key), Ok(value)) = (
                        general_purpose::STANDARD.decode(key_b64),
                        general_purpose::STANDARD.decode(value_b64),
                    ) {
                        store.insert(key, value);
                    }
                }
                ["delete", key_b64] => {
                    if let Ok(key) = general_purpose::STANDARD.decode(key_b64) {
                        store.remove(&key);
                    }
                }
                _ => {}
            }
        }

        // reopen file for append (because BufReader uses read-only)
        let log = OpenOptions::new()
            .append(true)
            .open(path)?;

        Ok(Kline { store, log })
    }


    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) -> std::io::Result<()> {
        let key_b64 = general_purpose::STANDARD.encode(&key);
        let value_b64 = general_purpose::STANDARD.encode(&value);

        writeln!(self.log, "put {} {}", key_b64, value_b64)?;
        self.log.flush()?;
        self.store.insert(key, value);
        Ok(())
    }


    pub fn get(&self, key: &[u8]) -> Option<&Vec<u8>> {
        self.store.get(key)
    }

     pub fn delete(&mut self, key: &[u8]) -> std::io::Result<()> {
        let key_b64 = general_purpose::STANDARD.encode(key);
        writeln!(self.log, "delete {}", key_b64)?;
        self.log.flush()?;
        self.store.remove(key);
        Ok(())
    }

    pub fn compact(&mut self) -> std::io::Result<()> {
        let temp_path = "kline.db.tmp";
        let mut temp_file = File::create(temp_path)?;

        for (key, value) in &self.store {
            let key_b64 = general_purpose::STANDARD.encode(key);
            let value_b64 = general_purpose::STANDARD.encode(value);
            writeln!(temp_file, "put {} {}", key_b64, value_b64)?;
        }

        std::fs::rename(temp_path, "kline.db")?;
        self.log = OpenOptions::new().append(true).open("kline.db")?;
        Ok(())
    }

    pub fn keys(&self) -> Vec<&Vec<u8>> {
        self.store.keys().collect()
    }
    
    pub fn clear(&mut self) -> std::io::Result<()> {
        self.store.clear();
        self.compact() // write empty state to disk
    }
}
