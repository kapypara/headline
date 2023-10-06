use std::vec;

#[derive(Debug)]
pub struct State {
    pub sessions: Vec<String>,
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
            sessions: vec![],
            components: Components {
                navgation_bar: String::new(),
                footer: String::new(),
            },
        }
        
    }
}
