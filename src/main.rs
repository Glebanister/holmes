mod logic;

use logic::*;

fn main() {
    let if_rain_umbrella = follows(just("Rain"), just("Take an umbrella"));
    let if_umbrella_buy = follows(just("Take an umbrella"), just("Buy an umbrella"));
    let rain = just("Rain");
    let umbrella = deduce(if_rain_umbrella, rain).unwrap();
    let buy = deduce(if_umbrella_buy, umbrella).unwrap();
    println!("{:?}", buy);
}
