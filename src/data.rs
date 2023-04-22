
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum IlData {
    Float(f64),
    Symbol(String),
    String(String),
    Tuple(Vec<IlData>),
    List(Vec<IlData>),
    Pattern(IlPat),
    Code(Vec<Il>),
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
    CallDef(String),
    CallStack,
    Return,
    Exit,
}

#[derive(Debug)] 
pub struct ExeEnv {
    pub data_stack : Vec<IlData>,
    pub def_stack : Vec<HashMap<String, IlData>>,
}