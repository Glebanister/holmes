mod command_line_interface;
use cli::{Rc, RefCell};
use command_line_interface as cli;
mod logic;

struct Holmes {
    running: bool,
    facts: Vec<Box<logic::Statement>>,
}

mod parse {
    use super::logic;
    pub fn parse_fact(input: &str) -> Option<Box<logic::Statement>> {
        let v: Vec<&str> = input.split("->").collect();
        let mut names: Vec<&str> = Vec::new();
        for at in v.iter() {
            names.push(at.trim());
        }
        let v = names;
        let mut chain = {
            match v.last() {
                Some(last) => logic::just(last),
                _ => return None,
            }
        };
        for i in (0..(v.len() - 1)).rev() {
            let cur_statement = logic::just(v.get(i).expect("Invalid iteration"));
            chain = logic::follows(cur_statement, chain);
        }
        return Some(chain);
    }
}

impl Holmes {
    fn reduce(&mut self) {
        let last_fact = {
            match self.facts.last() {
                Some(f) => f,
                None => panic!("Can not reduce with last: no facts were added"),
            }
        };
        let mut new_facts: Vec<Box<logic::Statement>> = Vec::new();
        for f in self.facts.iter() {
            match logic::deduce(f.clone(), last_fact.clone()) {
                Some(new_fact) => new_facts.push(new_fact),
                None => match logic::deduce(last_fact.clone(), f.clone()) {
                    Some(new_fact) => new_facts.push(new_fact),
                    None => continue,
                },
            }
        }
        self.facts.append(&mut new_facts);
    }
    pub fn add_fact(&mut self, input: &str) -> String {
        match parse::parse_fact(input) {
            Some(result) => {
                self.facts.push(result);
                self.reduce();
                String::new()
            }
            None => String::from("Can not parse input"),
        }
    }
    pub fn ask(&self, input: &str) -> String {
        match parse::parse_fact(input) {
            Some(fact) => match self.facts.iter().position(|x| *x == fact) {
                Some(_) => String::from("True"),
                None => String::from("False"),
            },
            None => String::from("Can not parse input"),
        }
    }
}

struct AddFactHandler {
    holmes: Rc<RefCell<Holmes>>,
}

impl cli::InputHandler<Holmes> for AddFactHandler {
    fn handle(&self, input: &String) -> Option<String> {
        if input.chars().next().unwrap() == '!' {
            Some((*self.holmes.borrow_mut()).add_fact(input.get(1..).unwrap()))
        } else {
            None
        }
    }
}

struct AskFactHandler {
    holmes: Rc<RefCell<Holmes>>,
}

impl cli::InputHandler<Holmes> for AskFactHandler {
    fn handle(&self, input: &String) -> Option<String> {
        if input.chars().next().unwrap() == '?' {
            Some((*self.holmes.borrow_mut()).ask(input.get(1..).unwrap()))
        } else {
            None
        }
    }
}

impl cli::StoppableInterface for Holmes {
    fn stop(&mut self) {
        self.running = false;
    }

    fn is_running(&self) -> bool {
        return self.running;
    }
}

fn main() {
    let holmes = Rc::new(RefCell::new(Holmes {
        running: true,
        facts: Vec::new(),
    }));
    let holmes_interface = cli::CommandLineInterface::new(
        String::from(">> "),
        String::from("Welcome to holmes!"),
        vec![
            Box::new(AddFactHandler {
                holmes: holmes.clone(),
            }),
            Box::new(AskFactHandler {
                holmes: holmes.clone(),
            }),
            Box::new(cli::ExitHandler {
                interface: holmes.clone(),
            }),
        ],
        holmes,
    );
    holmes_interface.run();
}
