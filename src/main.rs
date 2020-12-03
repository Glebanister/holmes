mod logic {
    pub enum Fact {
        Implies(Box<Fact>, Box<Fact>),
        And(Box<Fact>, Box<Fact>),
        Or(Box<Fact>, Box<Fact>),
        Not(Box<Fact>),
        Atom(String),
    }
    pub type FactBox = Box<Fact>;
    pub fn atom(name: &str) -> FactBox {
        let name = String::from(name);
        Box::new(Fact::Atom(name))
    }
    pub fn implies(a: FactBox, b: FactBox) -> FactBox {
        Box::new(Fact::Implies(a, b))
    }
    pub fn and(a: FactBox, b: FactBox) -> FactBox {
        Box::new(Fact::And(a, b))
    }
    pub fn or(a: FactBox, b: FactBox) -> FactBox {
        Box::new(Fact::Or(a, b))
    }
    pub fn not(a: FactBox) -> FactBox {
        Box::new(Fact::Not(a))
    }
}

use logic::*;

fn main() {
    let x = implies(atom("It is raining"), atom("I should get an umbrella"));
}
