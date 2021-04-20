use rand::Rng;

pub struct Dice {
    pub sides: u16
}

impl Dice {
    pub fn roll(&self) -> u16 {
        let mut rng = rand::thread_rng();
        if self.sides <= 1 {
            return 1;
        }
        return rng.gen_range(1..self.sides);
    }
}

pub struct DiceRoll {
    pub dice_type: Dice,
    pub dice_count: u16
}

fn str_to_int(s: &str, fallback: u16) -> u16 {
    let num : u16 = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => fallback
    };
    return num;
}


impl DiceRoll {
    pub fn roll(&self) {
        println!("{}w{}", self.dice_count, self.dice_type.sides);
        let mut sum : u16 = 0;
        for _i in 0..self.dice_count {
            let role = &self.dice_type.roll();
            sum = sum + role;
            print!("{} ", role);
        }
        print!(" = {}\n", sum);
    }

    fn split_by(to_parse: &String, s: &str) -> DiceRoll {
        let d: Vec<&str> = to_parse.split(s).collect();
        return DiceRoll { dice_type: Dice {sides: str_to_int(d[1], 6)}, dice_count: str_to_int(d[0], 1)};
    }

    pub fn for_string(to_parse: &String) -> DiceRoll {
        if to_parse.is_empty() {
            return DiceRoll { dice_type: Dice {sides: 6}, dice_count: 2};  
        }
        if to_parse.contains("w") {
            return DiceRoll::split_by(to_parse, "w");
        } else if to_parse.contains("d") {
            return DiceRoll::split_by(to_parse, "d");
        } 
        return DiceRoll { dice_type: Dice {sides: str_to_int(to_parse, 6) }, dice_count: 1};
    }

}

