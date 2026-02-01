/*
    Let's model a road trip!
*/

/*
    Define a `start_trip` function that creates and returns
    a String of "The plan is..."
*/
fn start_trip() -> String {
    String::from("The plan is...")
}

/*
    Define a `visit_philadelphia` function that concatenates
    the text "Philadephia" to the end of the String.
*/
fn visit_philadelphia(trip: &mut String) {
    trip.push_str("Philadephia");
}

/*
    Define a `visit_new_york` function that concatenates the
    text "New York" to the end of the String. Invoke the function
    in `main`. Repeat the previous logic to concatenate " and "
    to the end of the String.
*/
fn visit_new_york(trip: &mut String) {
    trip.push_str("New York");
}

/*
    Define a `visit_boston` function that concatenates the
    text "Boston." to the end of the String. 
*/
fn visit_boston(trip: &mut String) {
    trip.push_str("Boston.");
}

/*
    Define a `show_itinerary` function that will print out
    the final version of the String. Find a way to do so
    without transferring ownership.
*/
fn show_itinerary(trip: &String) {
    println!("Itinerary:{trip}");
}

fn main() {
    /*
        Invoke the `start_trip` function in `main` and save its
        return value to a `trip` variable.
    */
    let mut trip = start_trip();
    println!("{trip}");

    /*
        We want to pass the String to three separate functions
        that will mutate the String without transferring ownership.
    */

    /*
    Invoke the function visit_philadelphia in `main`. Then, 
    invoke `push_str` on the String to concatenate the 
    content " and " to the end. Make sure to include the spaces.
    */
    visit_philadelphia(&mut trip);
    println!("{trip}");
    trip.push_str(" and ");

    /*
        Invoke the function visit_new_york in `main`. 
        Repeat the previous logic to concatenate " and "
        to the end of the String.
    */
    visit_new_york(&mut trip);
    println!("{trip}");
    trip.push_str(" and ");

    /*
        Invoke the function in `main`. Concatenate a period 
        to the end of the String/sentence.
    */
    visit_boston(&mut trip);

    /*
        Invoke `show_itinerary`. The final output should be:
    
        "The plan is...Philadelphia and New York and Boston."
    */
    show_itinerary(&trip);
}
