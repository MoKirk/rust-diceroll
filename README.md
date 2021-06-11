# Rust Diceroller

Do you need a random number from your cli? As a gamemaster, I sometimes use this in order to generate a random dice roll.

## Installation

Basically you have to build the module using rust. Nothing special here.

``` 
$ cargo build --release
$ ./target/release/dice_roller
```

## Modes

The diceroller has two modes. You can provide a set of dice via the command line or use the interactive mode. Here are the ways to use it (both examples are for 2d6):


### CLI Mode
``` 
$ ./target/release/dice_roller 2d6 
2d6
2 1  = 3
```

### INTERACTIVE Mode
``` 
$ ./target/release/dice_roller 
2d6
2d6
5 5  = 10
q
```

