use std::thread;

// todo: put in its own file
struct Philosopher {
    name: String,
}

impl Philosopher {
    // class/static methods ('associated functions' in rustspeak)
     fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
    
    // member methods (&self)
    pub fn eat(&self) {
        println!("{} is eating.", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let names = vec![
        "Gilles Deleuze",
        "John Locke",
        "Hurley from Lost",
        "Barry White"
    ];
    // how am i doing ruby magic in a low level language? what is going on man?
    let philosophers = names.iter().map( |name| Philosopher::new(name) );

    // into_iter means "take ownership of original objects"
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        // move annotation means closure takes ownership
        thread::spawn(move || {
            p.eat();
        })
    }).collect();

    // no .each yet - https://users.rust-lang.org/t/map-filter-but-no-each/965
    for philosopher_thread in handles {
        philosopher_thread.join().unwrap();
    }
}

