use graphqxl_parser::BlockDef;
use std::collections::HashMap;

pub(crate) struct BlockDefStore<'a> {
    hash_maps: Vec<&'a HashMap<String, BlockDef>>,
}

impl<'a> From<&'a HashMap<String, BlockDef>> for BlockDefStore<'a> {
    fn from(value: &'a HashMap<String, BlockDef>) -> Self {
        Self {
            hash_maps: vec![value],
        }
    }
}

impl<'a> From<Vec<&'a HashMap<String, BlockDef>>> for BlockDefStore<'a> {
    fn from(value: Vec<&'a HashMap<String, BlockDef>>) -> Self {
        Self { hash_maps: value }
    }
}

impl<'a> BlockDefStore<'a> {
    pub(crate) fn get(&self, key: &str) -> Option<&BlockDef> {
        for hash_map in self.hash_maps.iter() {
            if let Some(result) = hash_map.get(key) {
                return Some(result);
            }
        }
        None
    }
}
