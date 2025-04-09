mod abstraction;
use abstraction::{AirplaneSim, AirplaneBooking, FlightSimulation, BookingSystem};

fn main() {
    // Create an instance of AirplaneSim
    let sim = AirplaneSim { altitude: 3500.0, speed: 850.0 };
    println!("Altitude: {} m, Speed: {} km/h", sim.get_altitude(), sim.get_speed());

    // Create an instance of AirplaneBooking
    let booking = AirplaneBooking { seat_map: String::from("1A 1B 2A 2B"), available_seats: 3 };
    println!("Seat Map: {}, Available Seats: {}", booking.get_seat_map(), booking.get_available_seats());
}
