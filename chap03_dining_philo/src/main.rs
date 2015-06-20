// todo: put in its own file
mod models {
    pub struct Philosopher {
        name: String,
    }

    impl Philosopher {
        // class/static methods ('associated functions' in rustspeak)
        pub fn new(name: &str) -> Philosopher {
            Philosopher {
                name: name.to_string(),
            }
        }
        
        // member methods (&self)
        pub fn eat(&self) {
            println!("{} is done eating.", self.name);
        }
    }
}

use models::Philosopher;

fn main() {
    let names = vec![
        "Gilles Deleuze",
        "John Locke",
        "Hurley from Lost",
        "Barry White"
    ];
    // how am i doing ruby magic in a low level language? what is going on man?
    let philosophers = names.iter().map( |name| Philosopher::new(name) );

    // no .each yet - https://users.rust-lang.org/t/map-filter-but-no-each/965
    for philosopher in philosophers {
        philosopher.eat();
    }
    println!("Hello, world!");
}
