use std::env;
use std::io;

mod dice;

use dice::DiceRoll;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        for n in 1..args.len() {
            let roll = DiceRoll::for_string(&args[n]);
            roll.roll();
        }
        return;
    }
    println!("Please input your dice");

    loop {
        let mut userinput = String::new();
        io::stdin().read_line(&mut userinput).expect("could not read");
        if userinput.is_empty() || userinput.contains("q") || userinput.contains("e") {
            return;
        }
        let roll = DiceRoll::for_string(&userinput);
        roll.roll();
    }
}
