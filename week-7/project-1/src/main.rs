use std::io;

fn trapezium(){

     println!("Enter height:");
     let mut height = String::new();
     io::stdin().read_line(&mut height).expect("Failed to read input");
     let height:f32 = height.trim().parse().expect("Failed to read input");

     println!("Enter base1:");
     let mut base1 = String::new();
     io::stdin().read_line(&mut base1).expect("Failed to read input");
     let base1:f32 = base1.trim().parse().expect("Failed to read input");

     println!("Enter base2:");
     let mut base2 = String::new();
     io::stdin().read_line(&mut base2).expect("Failed to read input");
     let base2:f32 = base2.trim().parse().expect("Failed to read input");

     let mut area:f32 = height/2.0 * (base1+base2);

     println!("Area of a Trapezium = {}",area);


}


fn rhombus(){

     println!("Enter diagonal1:");
     let mut diagonal1 = String::new();
     io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
     let diagonal1:f32 = diagonal1.trim().parse().expect("Failed to read input");

     println!("Enter diagonal2:");
     let mut diagonal2 = String::new();
     io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
     let diagonal2:f32 = diagonal2.trim().parse().expect("Failed to read input"); 

     let mut area:f32 = 1.0/2.0 * diagonal1 * diagonal2;

     println!("Area of rhombus = {}",area);
}

fn parallelogram(){

    println!("Enter base:");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Failed to read input");
    let base:f32 = base.trim().parse().expect("Failed to read input");

    println!("Enter altitude:");
    let mut altitude = String::new();
    io::stdin().read_line(&mut altitude).expect("Failed to read input");
    let altitude:f32 = altitude.trim().parse().expect("Failed to read input");

    let mut area:f32 = base * altitude;

    println!("Area of parallelogram = {}",area);
}

fn cube(){

    println!("Enter length of side:");
    let mut side_length = String::new();
    io::stdin().read_line(&mut side_length).expect("Failed to read input");
    let side_length:f32 = side_length.trim().parse().expect("Failed to read input");

    let mut area:f32 = 6.0 * side_length.powf(2.0);

    println!("Volume of cube = {}",area);
}

fn cylinder(){

    println!("Enter radius:");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("Failed to read input");
    let radius:f32 = radius.trim().parse().expect("Failed to read input");

    println!("Enter height");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height:f32 = height.trim().parse().expect("Failed to read input");

    let mut volume:f32 = 3.142 * radius.powf(2.0) * height;

    println!("Volume of cylinder = {}",volume);
}







    fn main() {

    loop{

    println!("Welcome!, select an equation:
              \n1. Area of Trapezium
              \n2. Area of Rhombus
              \n3. Area of Parallelogram
              \n4. Area of Cube
              \n5. Volume of Cylinder");

    // Read the user's choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice:f32 = choice.trim().parse().expect("Failed to read input");

    let arr = [trapezium,rhombus,parallelogram,cube,cylinder];

    if choice >= 1.0 && choice <= 5.0 {
        arr[(choice - 1.0) as usize]();
        break;
    }

    else {

        println!("Invalid Input");
        continue;
    }
    }
}














    
    




    





    