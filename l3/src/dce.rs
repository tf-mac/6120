use std::collections::HashSet;

use crate::program::{Function, Instrs, Program};

fn func_dce(mut func: Function) -> Function {
    let mut used = HashSet::new();
    let mut assigned = HashSet::new();

    func.instrs.reverse();
    func.instrs.retain(|x| match x {
        Instrs::Label(_) => {
            used.clear();
            assigned.clear();
            true
        },

        Instrs::Instruction(i) => {
            match &i.dest {
                Some(dest) if used.contains(dest) => {
                    used.remove(dest);
                    assigned.insert(dest.clone());
                    if let Some(args) = &i.args {
                        for a in args { used.insert(a.clone()); }
                    }
                    true
                }
                Some(dest) if assigned.contains(dest) => {
                    false
                }
                Some(dest) => {
                    assigned.insert(dest.clone());
                    if let Some(args) = &i.args {
                        for a in args { used.insert(a.clone()); }
                    }
                    true
                }
                None => {
                    if let Some(args) = &i.args {
                        for a in args { used.insert(a.clone()); }
                    }
                    true
                }
            }
        }
    });
    func.instrs.reverse();
    func
}

pub fn dce(mut program: Program) -> Program {
    program.functions = program.functions
        .into_iter()
        .map(|mut func| {
            loop {
                let before = func.instrs.len();
                func = func_dce(func);
                let after = func.instrs.len();
                if before == after {
                    break func;
                }
            }
        })
        .collect();
    program
}