use std::cmp;
use std::io::{self, Write};
use std::vec;
use std::{cell::RefCell, rc::Rc};

trait StoppableInterface {
    fn stop(&mut self);
    fn is_running(&self) -> bool;
}

trait InputHandler<T: StoppableInterface> {
    fn handle(&self, input: &String) -> Option<String>;
}

struct CommandLineInterface<T: StoppableInterface> {
    prefix: String,
    welcome_message: String,
    handlers: Vec<Box<dyn InputHandler<T>>>,
    interface: Rc<RefCell<T>>,
}

struct UndefinedInputHandler;
impl<T: StoppableInterface> InputHandler<T> for UndefinedInputHandler {
    fn handle(&self, input: &String) -> Option<String> {
        return Some(String::from("Undefined input: ") + input);
    }
}

struct ExitHandler<T: StoppableInterface> {
    interface: Rc<RefCell<T>>,
}
impl<T: StoppableInterface> InputHandler<T> for ExitHandler<T> {
    fn handle(&self, input: &String) -> Option<String> {
        let exit_command = String::from("exit");
        match input.cmp(&exit_command) {
            cmp::Ordering::Equal => {
                self.interface.borrow_mut().stop();
                return Some(String::new());
            }
            _ => return None,
        }
    }
}

impl<T: StoppableInterface> CommandLineInterface<T> {
    fn run(&self) -> () {
        println!("{}", self.welcome_message);
        while (*self.interface.borrow_mut()).is_running() {
            print!("{}", self.prefix);
            io::stdout().flush().unwrap();
            let s = {
                let mut input_string = String::new();
                io::stdin()
                    .read_line(&mut input_string)
                    .expect("Can not read from stdin");
                input_string.trim().to_string()
            };
            for h in self.handlers.iter() {
                match h.handle(&s) {
                    None => continue,
                    Some(return_value) => {
                        if !return_value.is_empty() {
                            println!("{}", return_value);
                        }
                        break;
                    }
                }
            }
        }
    }

    fn new(
        prefix: String,
        welcome_message: String,
        handlers: Vec<Box<dyn InputHandler<T>>>,
        interface: Rc<RefCell<T>>,
    ) -> CommandLineInterface<T> {
        let mut mutable_handlers = handlers;
        mutable_handlers.push(Box::new(UndefinedInputHandler));
        return CommandLineInterface {
            prefix: prefix,
            welcome_message: welcome_message,
            handlers: mutable_handlers,
            interface: interface,
        };
    }
}

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
    let counter = Rc::new(RefCell::new(Counter {
        count: 0,
        running: true,
    }));
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
