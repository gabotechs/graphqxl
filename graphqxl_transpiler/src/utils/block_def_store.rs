use graphqxl_parser::BlockDef;
use std::collections::HashMap;

pub(crate) struct BlockDefStore<'a> {
    hash_maps: Vec<&'a HashMap<String, BlockDef>>,
}

// TODO: This should be done with macros, but I don't know how to use them 🤷🏼‍
impl<'a> From<&'a HashMap<String, BlockDef>> for BlockDefStore<'a> {
    fn from(value: &'a HashMap<String, BlockDef>) -> Self {
        Self {
            hash_maps: vec![value],
        }
    }
}

impl<'a> From<(&'a HashMap<String, BlockDef>, &'a HashMap<String, BlockDef>)>
    for BlockDefStore<'a>
{
    fn from(value: (&'a HashMap<String, BlockDef>, &'a HashMap<String, BlockDef>)) -> Self {
        Self {
            hash_maps: vec![value.0, value.1],
        }
    }
}

impl<'a>
    From<(
        &'a HashMap<String, BlockDef>,
        &'a HashMap<String, BlockDef>,
        &'a HashMap<String, BlockDef>,
    )> for BlockDefStore<'a>
{
    fn from(
        value: (
            &'a HashMap<String, BlockDef>,
            &'a HashMap<String, BlockDef>,
            &'a HashMap<String, BlockDef>,
        ),
    ) -> Self {
        Self {
            hash_maps: vec![value.0, value.1, value.2],
        }
    }
}

impl<'a>
    From<(
        &'a HashMap<String, BlockDef>,
        &'a HashMap<String, BlockDef>,
        &'a HashMap<String, BlockDef>,
        &'a HashMap<String, BlockDef>,
    )> for BlockDefStore<'a>
{
    fn from(
        value: (
            &'a HashMap<String, BlockDef>,
            &'a HashMap<String, BlockDef>,
            &'a HashMap<String, BlockDef>,
            &'a HashMap<String, BlockDef>,
        ),
    ) -> Self {
        Self {
            hash_maps: vec![value.0, value.1, value.2, value.3],
        }
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
