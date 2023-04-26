
mod data;
mod machine;

fn main() {

    use crate::data::*;

    use std::collections::HashMap;
    use std::rc::Rc;

    let mut defs = HashMap::new();
    let mut dict = HashMap::new();

    let blarg = Rc::new(Word::Il(vec![Il::Push(IlData::Float(1.0)), Il::Push(IlData::Float(1.0)), Il::TupleCons(2)]));
    dict.insert("blarg".into(), blarg.clone());

    let other = Rc::new(Word::Func(vec![blarg.clone(), blarg.clone()]));

    let main = Rc::new(Word::Func(vec![other.clone(), blarg.clone()]));
    dict.insert("main".into(), main);

    let result = machine::execute("main".into(), &mut dict, &mut defs);

    println!("{:?}", result);
}
