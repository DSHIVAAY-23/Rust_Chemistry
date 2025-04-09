
struct Car {
    is_engine_on: bool,
    speed: u32,
}

impl Car {
    fn new() -> Self {
        Self {
            is_engine_on: false,
            speed: 0,
        }
    }

    fn start_engine(&mut self) {
        self.is_engine_on = true;
        println!("Engine started!");
    }

    fn accelerate(&mut self, increase_by: u32) {
        if self.is_engine_on {
            self.speed += increase_by;
            println!("Speed increased to {} km/h", self.speed);
        } else {
            println!("Can't accelerate. Start the engine first!");
        }
    }

    fn get_speed(&self) -> u32 {
        self.speed
    }
}
