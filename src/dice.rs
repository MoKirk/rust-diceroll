use rand::Rng;

pub struct Dice {
    pub sides: u8
}

impl Dice {
    pub fn roll(&self) -> u8 {
        let mut rng = rand::thread_rng();
        return rng.gen_range(1..self.sides);
    }
}

pub struct DiceRoll {
    pub dice_type: Dice,
    pub dice_count: u8
}

impl DiceRoll {
    pub fn roll(&self) {
        println!("{}w{}", self.dice_count, self.dice_type.sides);
        let mut sum : u8 = 0;
        for _i in 0..self.dice_count {
            let role = &self.dice_type.roll();
            sum = sum + role;
            print!("{} ", role);
        }
        print!(" = {}\n", sum);
    }

    pub fn for_string(to_parse: &String) -> DiceRoll {
        // TODO : THIS FUNCTION HAS TO BETTER CHECK THE INPUT
        if to_parse.is_empty() {
            return DiceRoll { dice_type: Dice {sides: 6}, dice_count: 2};  // TODO error handling
        }
        if to_parse.contains("w") {
            let d: Vec<&str> = to_parse.split("w").collect();
            return DiceRoll { dice_type: Dice {sides: d[1].trim().parse().expect("not a number")}, dice_count: d[0].trim().parse().expect("not a number")};
        } else if to_parse.contains("d") {
            let d: Vec<&str> = to_parse.split("d").collect();
            return DiceRoll { dice_type: Dice {sides: d[1].trim().parse().expect("not a number")}, dice_count: d[0].trim().parse().expect("not a number")};
        } 
        return DiceRoll { dice_type: Dice {sides: to_parse.trim().parse().expect("not a number")}, dice_count: 1};
    }

}

