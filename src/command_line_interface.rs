use std::cmp;
use std::io::{self, Write};
pub use std::{cell::RefCell, rc::Rc};

pub trait StoppableInterface {
    fn stop(&mut self);
    fn is_running(&self) -> bool;
}

pub trait InputHandler<T: StoppableInterface> {
    fn handle(&self, input: &String) -> Option<String>;
}

pub struct CommandLineInterface<T: StoppableInterface> {
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

pub struct ExitHandler<T: StoppableInterface> {
    pub interface: Rc<RefCell<T>>,
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
    pub fn run(&self) -> () {
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

    pub fn new(
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
