use std::collections::{HashMap, HashSet};

use crate::{cfg::BasicBlock, program::Arg};

pub fn reaching(cfg: HashMap<String, Vec<String>>, blocks: HashMap<String, BasicBlock>, entry: String, args: Vec<Arg>) -> (HashMap<String, HashSet<String>>, HashMap<String, HashSet<String>>) {
    let mut input = HashMap::new();
    let mut output = HashMap::new();
    let mut preds = HashMap::new();
    for (k, v) in cfg.iter() {
        for i in v {
            preds.entry(i.clone())
                .or_insert_with(Vec::new)
                .push(k.clone());
        }
    }
    let mut init = HashSet::new();
    for i in args {
        init.insert(i.name);
    }
    input.insert(entry, init);
    
    for i in blocks.keys() {
        output.insert(i.to_string(), HashSet::new());
    }
    let mut worklist: Vec<String> = blocks.keys().cloned().collect();
    while worklist.len() > 0 {
        match worklist.last() {
            Some(i) => {
                input.insert(i.to_string(), match preds.get(i) {
                    Some(list) => {
                        let mut set = HashSet::new();
                        for i in list {
                            match output.get(i) {
                                None => (),
                                Some(j) => {
                                    set = set.union(j).cloned().collect();
                                }
                            }
                        };
                        set
                    },
                    None => HashSet::new()
                });
                let prev = output.get(i).cloned();
                output.insert(i.to_string(), HashSet::new());
                match output.get(i) {
                    j if j.cloned() == prev => {
                        match cfg.get(i) {
                            None => (),
                            Some(list) => {
                                for k in list {
                                    worklist.push(k.to_string());
                                }
                            }
                        }
                    }
                    _ => ()
                }
            }
            None => {break;}
        }

    }
    (input, output)
}