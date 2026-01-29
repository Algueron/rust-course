/*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.
 
Cast the i32 to an i16 integer and assign the result
to a separate variable.
 
Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.
 
Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.
 
Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.
 
Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.
 
Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.
 
Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/

fn main() {
    let leet: i32 = 1_337;
    let leet16 = leet as i16;
    println!("{leet16}");

    let fvalue = 3.1415;
    println!("{fvalue:.3}");

    let with_milk: bool = true;
    let with_sugar: bool = false;

    let is_my_type_of_coffee = with_milk && with_sugar;
    let is_acceptable_coffee = with_milk || with_sugar;
    println!("{is_my_type_of_coffee},{is_acceptable_coffee}");

    let int_array = [ 34, 95, 45, 67];
    println!("{int_array:?}");

    let my_tuple = (leet16, fvalue, is_my_type_of_coffee, int_array);
    println!("{my_tuple:#?}");
}
