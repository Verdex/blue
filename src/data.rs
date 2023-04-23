
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum IlData {
    Float(f64),
    Symbol(String),
    String(String),
    Tuple(Vec<IlData>),
    List(Vec<IlData>),
    Pattern(IlPat),
}

#[derive(Debug, Clone)]
pub enum IlPat {
    Float(f64),
    Symbol(String),
    UnboundVariable(String),
    Tuple(Vec<IlPat>),
}

#[derive(Debug, Clone)]
pub enum Il {
    Push(IlData),
    TupleCons(usize),
    Match,
    Def,
    Exit,
}

pub enum Word {
    Il(Vec<Il>),
    Func(Vec<Rc<Word>>),
}

#[derive(Debug)] 
pub struct ExeResult<'a> {
    pub data_stack : Vec<IlData>,
    pub def_stack : DefStack<'a>,
}

#[derive(Debug)]
pub struct DefStack<'a> {
    base : &'a mut HashMap<String, IlData>,
    stack : Vec<HashMap<String, IlData>>,
}

impl<'a> DefStack<'a> {
    pub fn new(base : &'a mut HashMap<String, IlData>) -> Self {
        DefStack { base, stack : vec![] }
    }

    pub fn get(&self, name : &String) -> Option<&IlData> {
        let target = self.stack.iter().rev().find(|map| map.contains_key(name));
        match target { 
            Some(map) => map.get(name),
            None => self.base.get(name),
        }
    } 

    pub fn set(&mut self, name : String, data : IlData) {
        if self.stack.len() > 0 {
            let last = self.stack.len() - 1;
            self.stack[last].insert(name, data);
        }
        else {
            self.base.insert(name, data);
        }
    }

    pub fn push(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}