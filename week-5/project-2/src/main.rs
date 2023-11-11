use std::io;

fn main() {

    let mut age = String::new();
    let mut employee_experience = String::new();

    println!("Enter your age");
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Failed to input");

    println!("Enter if employee is Experienced or Inexperienced");
    io::stdin().read_line(&mut employee_experience).expect("Failed to read input");
    let employee_experience = employee_experience.trim();

    if age >= 40 && employee_experience == "Experienced" 
    {
        println!("Incentive = 1,560,000 NGN");
     }    

     else if age >= 30 && age < 40 && employee_experience == "Experienced"
    {
        println!("Incentive = 1,480,000 NGN");
    }

     else if age < 30 && employee_experience == "Experienced"
    {
         println!("Incentive = 1,300,000 NGN");
    } 

      else if employee_experience == "Inexperienced"
    {
        println!("Incentive = 100,000 NGN");
     }   

         






    

}
