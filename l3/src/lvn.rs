use std::collections::HashMap;

use crate::program::{Function, Instrs, Instruction, Program, Value};

// This entire function is unfortunately spaghetti code, due to not using basic blocks explicitly
// Its functional, but if I were you I'd just remake it from scratch
fn func_lvn(mut func: Function) -> Function {
    let mut table: HashMap<Vec<String>, String> = HashMap::new();
    let mut var2num: HashMap<String, usize> = HashMap::new();
    let mut num2var: HashMap<usize, String> = HashMap::new();
    let mut fresh_id: usize = 0;
    let mut rename_map: HashMap<String, String> = HashMap::new();

    let len = func.instrs.len();
    for idx in 0..len {
        let instr_ptr: *mut Instrs = &mut func.instrs[idx];
        unsafe {
            match instr_ptr.as_mut().unwrap() {
                Instrs::Label(_) => {
                    table.clear();
                    var2num.clear();
                    num2var.clear();
                    rename_map.clear();
                }
                Instrs::Instruction(i) if ["jmp", "ret", "br"].contains(&i.op.as_str()) => {
                    if let Some(ref mut args) = i.args {
                        for arg in args {
                            if let Some(new_name) = rename_map.get(arg) {
                                *arg = new_name.clone();
                            }
                        }
                    }
                    table.clear();
                    var2num.clear();
                    num2var.clear();
                    rename_map.clear();
                }
                Instrs::Instruction(i) => {
                    let mut updated_args = Vec::new();
                    if let Some(args) = &i.args {
                        for arg in args {
                            let actual_arg = rename_map.get(arg).unwrap_or(arg);
                            updated_args.push(actual_arg.clone());
                        }
                    }

                    let mut value = Vec::new();
                    value.push(i.op.clone());
                    
                    match i.value {
                        Some(Value::Number(v)) => value.push(v.to_string()),
                        Some(Value::Bool(v)) => value.push(v.to_string()),
                        _ => ()
                    }

                    for arg in &updated_args {
                        if let Some(&n) = var2num.get(arg) {
                            value.push(n.to_string());
                        } else {
                            let id = var2num.len();
                            var2num.insert(arg.clone(), id);
                            num2var.insert(id, arg.clone());
                            value.push(id.to_string());
                        }
                    }

                    if let Some(dest) = &i.dest {
                        if let Some(prev_dest) = table.get(&value) {
                            let actual_prev = rename_map.get(prev_dest).unwrap_or(prev_dest);
                            *i = Instruction {
                                op: "id".to_string(),
                                dest: Some(dest.clone()),
                                dest_type: i.dest_type.clone(),
                                args: Some(vec![actual_prev.clone()]),
                                funcs: None,
                                labels: None,
                                value: None,
                            };
                        } else {
                            let mut final_dest = dest.clone();
                            
                            let mut needs_rename = false;
                            for j in (idx + 1)..len {
                                match &func.instrs[j] {
                                    Instrs::Label(_) => break,
                                    Instrs::Instruction(ins) if ["jmp", "ret", "br"].contains(&ins.op.as_str()) => break,
                                    Instrs::Instruction(ins) => {
                                        if let Some(d2) = &ins.dest {
                                            if d2 == dest {
                                                needs_rename = true;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }

                            if needs_rename {
                                fresh_id += 1;
                                final_dest = format!("{}{}", dest, fresh_id);
                                rename_map.insert(dest.clone(), final_dest.clone());
                            }

                            i.dest = Some(final_dest.clone());
                            i.args = Some(updated_args);

                            let id = var2num.len();
                            var2num.insert(final_dest.clone(), id);
                            num2var.insert(id, final_dest.clone());
                            table.insert(value, final_dest);
                        }
                    } else {
                        i.args = Some(updated_args);
                    }
                }
            }
        }
    }

    // Second pass: apply all remaining renames
    for instr in &mut func.instrs {
        if let Instrs::Instruction(i) = instr {
            if let Some(ref mut args) = i.args {
                for arg in args {
                    if let Some(new_name) = rename_map.get(arg) {
                        *arg = new_name.clone();
                    }
                }
            }
        }
    }

    func
}

pub fn lvn(mut program: Program) -> Program {
    program.functions = program.functions
        .into_iter()
        .map(|func| func_lvn(func))
        .collect();
    program
}