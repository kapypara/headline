use std::vec;

use std::sync::Mutex;

#[derive(Debug)]
pub struct State {
    pub sessions: Vec<String>,
    pub components: Components,
}

#[derive(Clone, Debug)]
pub struct Components {
    pub navgation_bar: String,
}

impl State {
    pub fn new() -> State {
        State {
            sessions: vec![],
            components: Components { navgation_bar: String::new() },
        }
        
    }
}
