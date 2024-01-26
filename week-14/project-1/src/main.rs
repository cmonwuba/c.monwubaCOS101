use std::io;
use std::io::Read;


fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter Name:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let name:String = input1.trim().parse().expect("Failed to read input");

    println!("What is your Position?\nPress 1 for administrator\nPress 2 for project manager\nPress 3 for Employee\nPress 4 for Customer\nPress 5 for Vendor");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let post:i32 = input2.trim().parse().expect("Failed to read input");

    if post == 1 {
        admin();
    }
    else if post == 2 {
        project();
    }
    else if post == 3 {
        employee();
    }
    else if post == 4 {
        customer();
    }
    else if post == 5 {
        vendor();
    }
    else {
        println!("Not valid");
    }


}
   
fn admin(){

    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

fn project(){
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn employee(){
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn customer(){
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn vendor(){
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}