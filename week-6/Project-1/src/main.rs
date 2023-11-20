use std::io;

fn main() {

    let mut cand_name = String::new();
    let mut cand_email = String::new();
    let mut cand_department = String::new();
    let mut cand_state = String::new();
    let mut cand_classrep = String::new();
    let mut cand_level = String::new();
    let mut cand_cgpa = String::new();
    let mut cand_number = String::new();


    println!("Hello Candidate, Please enter your no: ");
    io::stdin().read_line(&mut cand_number).expect("Failed to read input");
    let cand_number:i32 = cand_number.trim().parse().expect("Failed to read input");

    if cand_number > 150 {
        println!("Oops only the first 150 can vote",);
    }

    else {

        println!("Are you a current class rep or not?");
        io::stdin().read_line(&mut cand_classrep).expect("Failed to read input");
        let cand_classrep = cand_classrep.trim().to_lowercase();

        if cand_classrep == "no" {
            println!("Sorry you are ineligible to vote");
    }


    else {
        println!("Enter your level: ");
    io::stdin().read_line(&mut cand_level).expect("Failed to read input");
    let cand_level:i32 = cand_level.trim().parse().expect("Failed to read input");

    if cand_level == 100 {
        println!("Sorry, you are ineligible to vote");
          return;
    }
    
    else {
        println!("Enter your CGPA: ");
    io::stdin().read_line(&mut cand_cgpa).expect("Failed to read input");
    let cand_cgpa = cand_cgpa.trim();

    if cand_cgpa <&4.0.to_string(){
        println!("Sorry, you are ineligible to vote");
        return;
    }

    else {
        println!("Fill in the form below");
    }

  }  

    println!("Enter your name: ");
    io::stdin().read_line(&mut cand_name).expect("Failed to read input");
    let cand_name = cand_name.trim();

    println!();

    println!("Enter your email: ");
    io::stdin().read_line(&mut cand_email).expect("Failed to read input");
    let cand_email = cand_email.trim();

       println!();

       println!("Enter your department: ");
       io::stdin().read_line(&mut cand_department).expect("Failed to read input");
       let cand_department = cand_department.trim();

        println!();

        println!("Enter your state of origin: ");
        io::stdin().read_line(&mut cand_state).expect("Failed to read input");
        let cand_state = cand_state.trim();

        println!();

        println!("YOU ARE ELIGIBLE FOR THE STUDENT COUNCIL");
    }
  } 




} 

    

    

    
    
    

    















        
    










    



    

