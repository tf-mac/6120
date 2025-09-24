use std::collections::HashMap;

use crate::program::{Function, Instr, Instruction};

pub type BasicBlock = Vec<Instr>;

pub fn build_blocks(function: Function) -> Vec<BasicBlock> {
    let mut blocks = Vec::new();
    let mut block = Vec::new();
    for i in function.instrs {
        match i {
            Instr::Instruction(j) => {
                match j {
                    Instruction::Jmp {..} | Instruction::Br {..} | Instruction::Ret {..} => {
                        block.push(Instr::Instruction(j));
                        blocks.push(block.clone());
                        block.clear();
                    },
                    _ => {
                        block.push(Instr::Instruction(j));
                    }
                };
            },
            Instr::Label(_) => {
                blocks.push(block.clone());
                block.clear();
                block.push(i);
            }
        }
    }
    blocks.push(block.clone());
    blocks
}

pub fn map_blocks(vec: Vec<BasicBlock>) -> (HashMap<String, BasicBlock>, HashMap<String, usize>, Vec<String>) {
    let mut map = HashMap::new();
    let mut index = HashMap::new();
    let mut vals = Vec::new();

    for (iindex, i) in vec.iter().enumerate() {
        match i.get(0) {
            Some(Instr::Label(l)) => {
                map.insert(l.label.clone(), i.clone());
                index.insert(l.label.clone(), iindex);
                vals.push(l.label.clone());
            },
            Some(Instr::Instruction(_)) => {
                map.insert(format!("b{}", map.len()), i.clone());
                index.insert(format!("b{}", index.len()), iindex);
                vals.push(format!("b{}", vals.len()));
            },
            None => {continue;}
        }
    }

    (map, index, vals)
}

pub fn build_cfg(map: HashMap<String, BasicBlock>, index: HashMap<String, usize>, vals: Vec<String>) -> HashMap<String, Vec<String>>{
    let mut cfg = HashMap::new();
    for (k, v) in map.iter() {
        match v.last() {
            Some(Instr::Instruction(Instruction::Br { labels, ..})) => {
                cfg.insert(k.to_string(), labels.to_vec());
            },
            Some(Instr::Instruction(Instruction::Jmp { labels, .. })) => {
                cfg.insert(k.to_string(), labels.to_vec());
            },
            Some(Instr::Instruction(Instruction::Ret { .. })) => {
                cfg.insert(k.to_string(), Vec::new());
            }
            Some(_)=> {
                if index[k] + 1 >= index.len() {
                    cfg.insert(k.to_string(), Vec::new());
                } else {
                    cfg.insert(k.to_string(), [vals[index[k] + 1].to_string()].to_vec());
                }
            },
            None => {continue;}
        }
    }
    cfg
}