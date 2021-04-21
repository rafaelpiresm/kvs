use std::collections::HashMap;

///KvStore eh a estrutura de dados básica
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    
    ///Creates KvStore
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    ///Busca conteúdo dentro da KvStore    
    pub fn get(&mut self, key: String) -> Option<String> {
        self.map.get(&key)
            .cloned()
    }

    ///Armazena conteúdo dentro da KvStore
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);       
    }

    ///Remove conteúdo de dentro da KvStore
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}