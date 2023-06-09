
use std::rc::Rc;
use std::collections::HashMap;

use crate::data::*;

// TODO mutable list of parsers
// TODO no more main
// TODO &str of input
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
                for instr in instrs {
                    match instr {
                        Il::Push(data) => {
                            data_stack.push(data.clone());
                        },
                        Il::TupleCons(count) => {
                            let s = data_stack.len() - count;
                            let params = data_stack.drain(s..).collect();
                            data_stack.push(IlData::Tuple(params));
                        },
                        Il::DefVar => { // (sym data -- )
                            let sym = data_stack.pop().unwrap(); // TODO
                            let data = data_stack.pop().unwrap(); // TODO

                            if let IlData::Symbol(name) = sym {
                                // TODO collision?
                                def_stack.set(name, data);
                            }
                            else {
                                // TODO
                            }
                        },
                        Il::DefWord => { // (sym list<sym> -- )
                            println!("def word entry");
                            let sym = data_stack.pop().unwrap(); // TODO
                            let def = data_stack.pop().unwrap(); // TODO

                            if let (IlData::Symbol(name), IlData::List(code)) = (sym, def) {
                                let mut func_addr = vec![];
                                for func_name in code.iter() {
                                    match func_name {
                                        IlData::Symbol(x) => {
                                            let addr = dict.get(x).unwrap(); // TODO
                                            func_addr.push(addr.clone());
                                        },
                                        _ => todo!(), // TODO unreachable?
                                    }
                                }
                                dict.insert(name, Rc::new(Word::Func(func_addr))); // TODO collision
                                println!("def word");
                            }
                            else {
                                // TODO
                                println!("else of def word");
                            }
                        },
                        Il::Exit => { break 'main_loop; },
                        _ => todo!(),
                    }
                }
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

fn slice<'a, T>(input : &'a Vec<T>) -> &'a [T] { &input[..] }
fn unbox<'a, T>(input : &'a Box<T> ) -> &'a T { &**input }