//a simple temperature unit converter
use std::io;
use std::cmp::Ordering;

fn main() {
    loop {
    //introduces the user to their options for unit_type
        println!("Select the unit you would like to convert from", );
        println!("Type 1 for Fahrenheit or 2 for Celsius", );
    //allows user input
        let mut unit = String::new();
    //reads user input
        io::stdin().read_line(&mut unit)
            .expect("Failed to read line");
    //states units used
        match unit.cmp (&unit) {
            //F=>C
            Ordering::Equal => {
                println!("You are converting from Fahrenheit to Celsius", );
                println!("Input the temperature", );
                //sets up input and reads it
                let mut v = String::new();
                io::stdin().read_line(&mut v)
                    .expect("Failed to read line");
                //math
                let v = v.trim().parse::<f32>().expect("That's not a number!");
                let v = v - 32.0;
                let v = v * 5.0 / 9.0;
                //says result
                println!("This is equal to {} degrees in Celsius
                ", v);
            }
            //C=>F
            Ordering::Greater => {
                println!("You are converting from Celsius to Fahrenheit", );
                println!("Input the temperature", );
                //sets up input and reads it
                let mut v = String::new();
                io::stdin().read_line(&mut v)
                    .expect("Failed to read line");
                //math
                let v = v.trim().parse::<f32>().expect("That's not a number!");
                let v = v * 18.0 / 10.0;
                let v = v + 32.0;
                //says result
                println!("This is equal to {} degrees in Fahrenheit
                ", v);
            }
            //use less ordering
            Ordering::Less => {
            }
        }
        }
}
