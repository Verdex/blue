
use std::collections::HashMap;

use crate::data::*;

pub fn execute(il : Vec<Il>) -> ExeEnv {
    let mut ip = 0;
    let mut data_stack : Vec<IlData> = vec![];
    let mut def_stack : Vec<HashMap<String, IlData>> = vec![HashMap::new()];

    // TODO runtime what to do if def stack or data stack doesn't have what you're after

    loop {
        if il.len() <= ip {
            // TODO runtime error
            panic!("instruction pointer out of range");
        }

        let i = &il[ip];

        match i {
            Il::Push(data) => {
                data_stack.push(data.clone());
                ip+=1;
            },
            Il::TupleCons(count) => {
                let params = data_stack.drain((data_stack.len() - count)..).collect::<Vec<_>>();
                data_stack.push(IlData::Tuple(params));
                ip+=1;
            },
            Il::Def => { // (str data -- )
                let name = data_stack.pop().unwrap(); 
                let data = data_stack.pop().unwrap();
                let current_env = {
                    let last = def_stack.len() - 1;
                    &mut def_stack[last]
                };
                if let IlData::String(s) = name {
                    current_env.insert(s, data); // TODO what semantics should redefinition have
                }
                else {
                    // TODO runtime error
                    panic!("TODO");
                }
                
                ip+=1;
            },
            Il::Exit => {
                break;
            }
            _ => todo!(),
        }
    }

    ExeEnv { data_stack, def_stack }
}