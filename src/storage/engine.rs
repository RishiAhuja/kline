use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub struct Kline {
    store: HashMap<String, String>,
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
                ["put", key, value] => {
                    store.insert(key.to_string(), value.to_string());
                }
                ["delete", key] => {
                    store.remove(*key);
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

    pub fn put(&mut self, key: String, value: String) -> std::io::Result<()> {
        writeln!(self.log, "put {} {}", key, value)?;
        self.log.flush()?;
        self.store.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &str) -> std::io::Result<()> {
        writeln!(self.log, "delete {}", key)?;
        self.log.flush()?;
        self.store.remove(key);
        Ok(())
    }

    pub fn compact(&mut self) -> std::io::Result<()> {
        let temp_path = "kline.db.tmp";
        let mut temp_file = File::create(temp_path)?;
        
        for (key, value) in &self.store {
            writeln!(temp_file, "put {} {}", key, value)?;
        }
        
        std::fs::rename(temp_path, "kline.db")?;
        self.log = OpenOptions::new().append(true).open("kline.db")?;
        Ok(())
    }
}
