/*
    Define a Flight struct with the following fields:
    - an `origin` field (String)
    - a `destination` field (String)
    - a `price` field (f64)
    - a `passengers` field (u32)
    
    Derive a Debug trait implementation for the Flight struct.
*/
#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32
}

impl Flight {

    /*
        Define a `new` constructor function that returns a new
        instance of a Flight.
    */
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Flight {
        Flight { origin, destination, price, passengers }
    }

    /*
        Define a `change_destination` method that accepts a new
        destination and overwrites the value of the `destination`
        field.
    */
    fn change_destination(&mut self, destination: String) -> &mut Self {
        self.destination = destination;
        self
    }

    /*
        Define a `increase_price` method that raises the value
        of the `price` by 20% (multiply the `price` field by 1.20).
        Make sure to save the new `price` field value.
    */
    fn increase_price(&mut self) -> &mut Self {
        self.price *= 1.2;
        self
    }

    /*
        Define a `itinerary` method that prints out both the
        `origin` and `destination` fields in the following format
        (origin -> destination).
    */
    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}

fn main() {
    /*
        Use the constructor function to create a new Flight instance
        in the main function. Invoke all of the defined methods.
        Print out the struct in Debug format to confirm the struct
        updates as you expect.
    */
    let mut paris_miami = Flight::new(String::from("Paris"), String::from("Miami"), 367., 4);
    println!("{paris_miami:#?}");

    paris_miami
        .change_destination(String::from("Los Angeles"))
        .increase_price()
        .itinerary();
    println!("{paris_miami:#?}");

    /*
        Use struct update syntax to copy the `price` and `passengers`
        fields to a new Flight struct instance. Make sure to provide
        new Strings for the remaining fields to ensure ownership
        doesn't transfer. Assign the new Flight to a separate variable.
    */
    let london_moscow = Flight {
        origin: String::from("London"),
        destination: String::from("Moscow"),
        ..paris_miami
    };
    println!("{london_moscow:#?}");
}
