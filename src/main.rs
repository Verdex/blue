
mod data;
mod machine;

fn main() {

    use crate::data::*;

    use std::collections::HashMap;
    use std::rc::Rc;

    let mut defs = HashMap::new();
    let mut dict = HashMap::new();

    let blarg = Rc::new(Word::Il(vec![]));
    dict.insert("blarg".into(), blarg.clone());

    let other = Rc::new(Word::Func(vec![blarg.clone(), blarg.clone()]));

    let main = Rc::new(Word::Func(vec![other.clone(), other.clone()]));
    dict.insert("main".into(), main);



    machine::execute("main".into(), &mut dict, &mut defs);
}
