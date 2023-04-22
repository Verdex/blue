
use std::borrow::Cow;
use std::collections::HashMap;

use crate::data::*;

pub fn execute(il : Vec<Il>) -> ExeEnv {
    let mut ip = 0;
    let mut data_stack : Vec<IlData> = vec![];
    let mut def_stack : Vec<HashMap<String, IlData>> = vec![HashMap::new()];
    let mut code_stack : Vec<Code> = vec![];

    let mut code : Cow<_> = Cow::from(il);

    // TODO runtime what to do if def stack or data stack doesn't have what you're after


    // TODO checkout purple

    loop {
        if code.len() <= ip {
            // TODO runtime error
            panic!("instruction pointer out of range");
        }

        match &code[ip] {
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
            Il::CallDef(name) => { 
                let env = def_stack.iter_mut().rev().find(|x| x.contains_key(name)).unwrap(); // TODO what if we can't find the function?

                if let IlData::Code(target_code) = &env[name] {
                    let mut t = Cow::from(target_code);
                    std::mem::swap(&mut t, &mut code);
                    code_stack.push(Code { prog: t, ip: ip });
                    def_stack.push(HashMap::new());
                    ip = 0;
                }
                else {
                    todo!();
                }

            },
            Il::CallStack => {

            },
            Il::Exit => {
                break;
            }
            _ => todo!(),
        }
    }

    ExeEnv { data_stack, def_stack }
}


struct Code<'a> {
    prog : Cow<'a, [Il]>,
    ip : usize,
}