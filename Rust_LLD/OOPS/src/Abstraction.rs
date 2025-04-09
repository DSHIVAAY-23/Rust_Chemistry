// Abstraction through traits
trait FlightSimulation {
    fn get_altitude(&self) -> f32;
    fn get_speed(&self) -> f32;
}

trait BookingSystem {
    fn get_seat_map(&self) -> String;
    fn get_available_seats(&self) -> u32;
}

// FlightSimulation Implementation
struct AirplaneSim {
    altitude: f32,
    speed: f32,
}

impl FlightSimulation for AirplaneSim {
    fn get_altitude(&self) -> f32 {
        self.altitude
    }
    fn get_speed(&self) -> f32 {
        self.speed
    }
}

// BookingSystem Implementation
struct AirplaneBooking {
    seat_map: String,
    available_seats: u32,
}

impl BookingSystem for AirplaneBooking {
    fn get_seat_map(&self) -> String {
        self.seat_map.clone()
    }
    fn get_available_seats(&self) -> u32 {
        self.available_seats
    }
}
