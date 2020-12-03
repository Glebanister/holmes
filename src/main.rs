mod logic {
    #[derive(PartialEq, Eq, Debug)]
    pub enum Statement {
        Follows(Box<Statement>, Box<Statement>),
        Or(Box<Statement>, Box<Statement>),
        And(Box<Statement>, Box<Statement>),
        Not(Box<Statement>),
        Just(String),
    }

    pub fn deduce(from: Box<Statement>, with: Box<Statement>) -> Option<Box<Statement>> {
        match *from {
            Statement::Follows(st_from, st_follow) => {
                if st_from == with {
                    Some(st_follow)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn follows(from: Box<Statement>, what: Box<Statement>) -> Box<Statement> {
        Box::new(Statement::Follows(from, what))
    }

    pub fn or(a: Box<Statement>, b: Box<Statement>) -> Box<Statement> {
        Box::new(Statement::Or(a, b))
    }

    pub fn and(a: Box<Statement>, b: Box<Statement>) -> Box<Statement> {
        Box::new(Statement::And(a, b))
    }
    pub fn not(a: Box<Statement>) -> Box<Statement> {
        Box::new(Statement::Not(a))
    }
    pub fn just(name: &str) -> Box<Statement> {
        let name = String::from(name);
        Box::new(Statement::Just(name))
    }
}

use logic::*;

fn main() {
    let if_rain_umbrella = follows(just("Rain"), just("Take an umbrella"));
    let if_umbrella_buy = follows(just("Take an umbrella"), just("Buy an umbrella"));
    let rain = just("Rain");
    let umbrella = deduce(if_rain_umbrella, rain).unwrap();
    let buy = deduce(if_umbrella_buy, umbrella).unwrap();
    println!("{:?}", buy);
}
