use std::io;

fn main() {
    let mut distance_in_miles = 80.0;
    let miles_to_km = 0.621371;
    let distance_in_km = distance_in_miles/miles_to_km;

    println!("distance in kilometers is {}km",distance_in_km);

    let mut time = 2.0;
    let mut speed = distance_in_km/time;

    println!("The speed of the first car is {}km/hr",speed);

    distance_in_miles = 120.0;
    let distance_in_km = distance_in_miles/miles_to_km;

    println!(" distance in 2nd kilometers is {}km",distance_in_km);

    time = 4.0;
    speed = distance_in_km/time;

    println!("The speed of the second car is {}km/hr",speed);

    println!("");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the value for Distance in Miles:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    let distance_in_km = a/miles_to_km;
    println!("This is your value for kilometers {}km",distance_in_km);

    println!("");

    println!("Enter the value for time:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    let speed = distance_in_km/b;

    print!("The speed of the vehicle is {}km/hr",speed);

}





