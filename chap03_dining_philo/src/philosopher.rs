mod dining_philosophers {
    mod models {

        pub struct Philosopher {
            name: String,
        }

        impl Philosopher {
            fn new(name: &str) -> Philosopher {
                Philosopher {
                    name: name.to_string(),
                }
            }
        }

    }
}
