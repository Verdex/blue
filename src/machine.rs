
use std::rc::Rc;
use std::collections::HashMap;

use crate::data::*;

pub fn execute<'a>( main : String
                  , dict : &mut HashMap<String, Rc<Word>>
                  , defs : &'a mut HashMap<String, IlData>
                  ) -> ExeResult<'a> {
    let mut data_stack : Vec<IlData> = vec![];
    let mut def_stack = DefStack::new(defs);
    let mut func_stack : Vec<(Rc<Word>, usize)> = vec![];

   
    let mut current_word : Rc<Word> = dict.get(&main).unwrap().clone(); // TODO
    let mut ip : usize = 0;


    'main_loop : loop {
        match &*current_word {
            Word::Func(words) if ip >= words.len() => {
                // End of word
                def_stack.pop();
                match func_stack.pop() {
                    Some((word, new_ip)) => { 
                        current_word = word;
                        ip = new_ip;
                    },
                    None => { break 'main_loop; },
                }
            },
            Word::Func(words) => {
                func_stack.push((current_word.clone(), ip + 1));
                def_stack.push();

                current_word = words[ip].clone();
                ip = 0;
            },
            Word::Il(instrs) => {
                println!("ip {}", ip);
                for instr in instrs {
                    match instr {
                        Il::Exit => { break 'main_loop; },
                        _ => todo!(),
                    }
                }
                // TODO def needs to go into last - 1 def_stack instead of last
                def_stack.pop();
                match func_stack.pop() {
                    Some((word, new_ip)) => { 
                        current_word = word;
                        ip = new_ip;
                    },
                    None => { break 'main_loop; },
                }
            },
        }
    }

    ExeResult { data_stack, def_stack }
}
