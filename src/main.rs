#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::io::Error;
use std::time::Duration;

fn main() {
    let mut hash_map: HashMap<String, String> = HashMap::<String, String>::new();
    (&mut hash_map).insert("foo".to_string(), "bar".to_string());
    let mut btree_map: BTreeMap<String, String> = BTreeMap::new();
    (&mut btree_map).insert("foo".to_string(), "bar".to_string());
    let mut queue: VecDeque<i32> = VecDeque::<i32>::new();
    for i in 1..=900 {
        if i % 68 == 0 {
            (&mut queue).push_back(i);
        }
    }
}
