mod command_line_interface;

use command_line_interface::{CommandLineInterface, ExitHandler, InputHandler, StoppableInterface};
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

struct Counter {
    count: i32,
    running: bool,
}

impl StoppableInterface for Counter {
    fn stop(&mut self) {
        self.running = false;
    }
    fn is_running(&self) -> bool {
        return self.running;
    }
}

impl Counter {
    fn increase(&mut self) {
        self.count += 1;
    }

    fn decrease(&mut self) {
        self.count -= 1;
    }

    fn get_count(&self) -> i32 {
        return self.count;
    }
}

struct IncreaseHandler {
    counter: Rc<RefCell<Counter>>,
}

impl InputHandler<Counter> for IncreaseHandler {
    fn handle(&self, input: &String) -> Option<String> {
        let plus = String::from("++");
        match input.cmp(&plus) {
            cmp::Ordering::Equal => {
                (*self.counter.borrow_mut()).increase();
                return Some(String::new());
            }
            _ => return None,
        };
    }
}

struct DecreaseHandler {
    counter: Rc<RefCell<Counter>>,
}

impl InputHandler<Counter> for DecreaseHandler {
    fn handle(&self, input: &String) -> Option<String> {
        let plus = String::from("--");
        match input.cmp(&plus) {
            cmp::Ordering::Equal => {
                (*self.counter.borrow_mut()).decrease();
                return Some(String::new());
            }
            _ => return None,
        };
    }
}

struct GetValueHandler {
    counter: Rc<RefCell<Counter>>,
}

impl InputHandler<Counter> for GetValueHandler {
    fn handle(&self, input: &String) -> Option<String> {
        let plus = String::from("??");
        match input.cmp(&plus) {
            cmp::Ordering::Equal => {
                return Some((*self.counter.borrow_mut()).get_count().to_string());
            }
            _ => return None,
        };
    }
}

fn main() {
    let running = true;
    let counter = Rc::new(RefCell::new(Counter { count: 0, running }));
    let counter_interface = CommandLineInterface::new(
        String::from(">> "),
        String::from("Welcome to counter!"),
        vec![
            Box::new(GetValueHandler {
                counter: counter.clone(),
            }),
            Box::new(IncreaseHandler {
                counter: counter.clone(),
            }),
            Box::new(DecreaseHandler {
                counter: counter.clone(),
            }),
            Box::new(ExitHandler {
                interface: counter.clone(),
            }),
        ],
        counter,
    );
    counter_interface.run();
}
