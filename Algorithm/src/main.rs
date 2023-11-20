use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();

    println!("Enter your name:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    

    println!("Enter your date of birth:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let dfb:i32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter Email Address:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    

    println!("Enter your Phone Number:");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let phone number:i32 = input4.trim().parse().expect("Not a valid number");

    println!("Enter No of Siblings:");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let no of siblings:i32 = input5.trim().parse().expect("Not a valid number");

    println!("Enter No of Children:");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let no of children:i32 = input6.trim().parse().expect("Not a valid number");

    println!("Enter Medical Diagnosis");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    
    println!("Enter Village of residence");
    io::stdin().read_line(&mut input8).expect("Not a valid string");

    let price_Alzheimer = 0.2 * 1_200_000.0;
    let discount = 1_200_000 - 240_000;
    

    if patient == "Alzheimer" && patient > 50 && patient no of children > 4 && patient village of residence == "Akpabom";

    {
        println!("Name: {}, diagnosis: {}, Amount after Discount: {}");
    }
    else if patient == "Arrythmia" == 30 && patient no of siblings > 4 && patient village of residence == "Ngbauji";

    let price_Arrythmia = 0.05 * 550_000;
    let discount = 550_000 - 27_500;
    {
        println!("Name: {}, diagnosis: {}, Amount after Discount: {}");
    }
    else if patient == "Chronic Kidney Disease(CKD)" > 40 && patient no of siblings/children > 3 && patient village of residence == "Atabrikang";

    let price_Arrythmia = 0.15 * 1_500_000;
    let discount = 1_500_000 - 225_000;
    {
        println!("Name: {}, diagnosis: {}, Amount after Discount: {}");
    }
    else if patient == "diabetes"> 28 && < 45 && patient no of children == 2-4 && patient village of residence == "Okorobilom";

    let price_diabetes = 0.10 * 800_000;
    let discount = 800_000 - 80_000;
    {
        println!("Name: {}, diagnosis: {}, Amount after the discount: {}");
    }







}







    

