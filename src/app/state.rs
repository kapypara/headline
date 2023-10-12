use std::collections::BTreeMap;
use std::sync::Mutex;

#[derive(Debug)]
pub struct State {
    pub sessions: Mutex<BTreeMap<Box<str>, usize>>,
    pub components: Components,
}

#[derive(Clone, Debug)]
pub struct Components {
    pub navgation_bar: String,
    pub footer: String,
}

impl State {
    pub fn new() -> State {
        State {
            sessions: BTreeMap::new().into(),
            components: Components {
                navgation_bar: String::new(),
                footer: String::new(),
            },
        }
        
    }

}
