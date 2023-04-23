
use std::rc::Rc;
use std::collections::HashMap;

use crate::data::*;

pub fn execute( main : String
              , dict : &mut HashMap<String, Rc<Word>>
              , defs : &mut HashMap<String, IlData>
              ) -> ExeResult {
    let mut data_stack : Vec<IlData> = vec![];
    let mut def_stack = DefStack::new(defs);
    let mut func_stack : Vec<(Rc<Word>, usize)> = vec![];

   
    let mut current_word = dict.get(&main).unwrap(); // TODO
    let mut ip : usize = 0;

    loop {
        /*if code.len() <= ip {
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
            Il::Exit => {
                break;
            }
            _ => todo!(),
        }*/
    }

    ExeResult { data_stack, def_stack }
}
