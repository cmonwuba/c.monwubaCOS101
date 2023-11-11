use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value for a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter value for b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter value for c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let v = b.powf(2.0);
    let _v_ = 4.0*a*c;
    let w = v - _v_;
    let _x = w.sqrt();
    let y = -b + _x;
    let _y = -b - _x;
    let _y_ = y / 2.0*a ;
    let z = _y / 2.0*a;
    
   println!("The roots of the quadratic equation: {},{}", _y_,z);

    let k = b.powf(2.0) - 4.0*a*c;
    let k_1= b.powf(2.0) - 4.0*a*c;
    let k_2 = b.powf(2.0) - 4.0*a*c;

    if k > 0.0
    {
        println!("There are two distinct real roots");
    }
    else if k_1 == 0.0
    {
        println!(" There is one repeated real root");
     }
     else if k_2 < 0.0
     {
        println!("There are no real solutions");
    }
    else
    {
        println!("The equation has no roots");
    }
}    
    















    

