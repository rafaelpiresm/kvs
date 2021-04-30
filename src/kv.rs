use std::collections::HashMap;
use std::path::Path;
use crate::{KvsError, Result};

///KvStore eh a estrutura de dados básica
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

///Soh pra buildar o teste
//pub enum Result<T> {}

impl KvStore {
    ///Creates KvStore
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    ///Busca conteúdo dentro da KvStore    
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    ///Armazena conteúdo dentro da KvStore
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key, value);
        Ok(())
    }

    ///Remove conteúdo de dentro da KvStore
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.map.remove(&key);
        Ok(())
    }

    ///Open a file 
    pub fn open(_path: &Path) -> Result<KvStore> {
        unimplemented!()
    }
}
