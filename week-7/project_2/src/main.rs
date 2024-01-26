
use std::io;

fn main() {

    //creates an empty array to add stuff  

    let mut users_inputs: Vec<String> = Vec::new();

    //Ask for number of sibs

    let mut input = String::new();


    println!("How many siblings do you have?");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _no_of_sibs:i32 = input.trim().parse().expect("Invalid input");

    let _a_nos:String = format!("No of siblings: {}",_no_of_sibs);

    users_inputs.push(_a_nos.to_string()); //pushes into empty array

    for i in 0.._no_of_sibs { //repeats form epending on no of siblings
    

    if _no_of_sibs >= 1 { //input name and age of each
            
        

                let mut input1 = String::new();
                let mut input2 = String::new();

                println!("Full name of sibling {}:",i + 1);
                io::stdin().read_line(&mut input1).expect("Failed to read input");
                let _names_of_sibs:String = input1.trim().parse().expect("Invalid input");

                println!("Age of sibling {}",i + 1);
                io::stdin().read_line(&mut input2).expect("Failed to read input");
                let _age_of_sib:i32 = input2.trim().parse().expect("Invalid input");

                let _a_naos:String = format!("Full name of sibling {}: {}",i + 1,_names_of_sibs);
                let _a_aos:String = format!("Age of sibling {}: {}",i + 1,_age_of_sib);

                users_inputs.push(_a_naos.to_string());
                users_inputs.push(_a_aos.to_string());

                        if _age_of_sib  >= 18 {

                            //if sib is older than 18
                            //ask if married or single-

                            let mut input3 = String::new();

                            println!("Relationship status of {}?\nInput 1 for single\nInput 2 for married",_names_of_sibs);
                            io::stdin().read_line(&mut input3).expect("Failed to read input");
                            let _rel_no:i32 = input3.trim().parse().expect("Invalid input");

                                    if _rel_no == 1 {//if single
                                                    //ask if student or worker

                                        let _rel = "Relationship Status: Single";
                                        let mut input4 = String::new();

                                        println!("Is He/She a student or worker?\nInput 1 for student\nInput 2 for worker");
                                        io::stdin().read_line(&mut input4).expect("Failed to read input");
                                        let _s_o_w:i32 = input4.trim().parse().expect("Invalid input");

                                        users_inputs.push(_rel.to_string());

                                                if _s_o_w == 1 { //if student
                                                                //ask for uni and course of study

                                                    let _occ = "Occupation Status: Student";            
                                                    let mut input5 = String::new();
                                                    let mut input6 = String::new();

                                                    println!("What university?");
                                                    io::stdin().read_line(&mut input5).expect("Failed to read input");
                                                    let _uni:String = input5.trim().parse().expect("Invalid input");

                                                    println!("What course of study?");
                                                    io::stdin().read_line(&mut input6).expect("Failed to read input");
                                                    let _cos:String = input6.trim().parse().expect("Invalid input");

                                                    let _a_uni:String = format!("Name of sibling {}'s University: {}",i + 1,_uni);
                                                    let _a_cos:String = format!("Course of Study for sibling {}: {}",i + 1,_cos);

                                                        users_inputs.push(_occ.to_string());
                                                        users_inputs.push(_a_uni.to_string());
                                                        users_inputs.push(_a_cos.to_string());

                                                }
                                                else if _s_o_w == 2 {

                                                    let _occ = "Occupation Status: Worker";

                                                    users_inputs.push(_occ.to_string());

                                                    println!("That is all");
                                                }
                                    }
                                    else if _rel_no == 2 {  //if sib is married
                                                        //ask for offspring and city family lives

                                        let _rel = "Relationship Status: Married";                
                                        let mut input7 = String::new();
                                        let mut input8 = String::new();

                                        println!("How many children do they have?");
                                        io::stdin().read_line(&mut input7).expect("Failed to read input");
                                        let _no_of_child:i32 = input7.trim().parse().expect("Invalid input");

                                        println!("Enter their family residence:");
                                        io::stdin().read_line(&mut input8).expect("Failed to read input");
                                        let _res:String = input8.trim().parse().expect("Invalid input");

                                        let _a_noc:String = format!("Number of Children: {}",_no_of_child);
                                        let _a_res:String = format!("Family Resience: {}",_res);

                                        users_inputs.push(_rel.to_string());
                                        users_inputs.push(_a_noc.to_string());
                                        users_inputs.push(_a_res.to_string());
                                    }
                        }
                        else if _age_of_sib < 18 {  //if the siblings is less than 18
                                                        //ask waec status

                            let mut input9 = String::new();

                            println!("Enter waec status: 1 for yes, 2 for No");
                            io::stdin().read_line(&mut input9).expect("Failed to read input");
                            let _waec:i32 = input9.trim().parse().expect("Invalid input");

                                        if _waec == 1 {  //if yes
                                                        //input sec sch

                                            let _waecval = "WAEC STATUS: COMPLETED";
                                            let mut input10 = String::new();

                                            println!("Enter Secondary school:");
                                            io::stdin().read_line(&mut input10).expect("Failed to read input");
                                            let _sec:String = input10.trim().parse().expect("Invalid input");

                                            let _a_sec:String = format!("Name of Secondary School: {}",_sec);

                                            users_inputs.push(_waecval.to_string());
                                            users_inputs.push(_a_sec.to_string());

                                        }
                                        else if _waec == 2 {   //if no
                                                               //input current class

                                            let _waecval = "WAEC STATUS: NOT COMPLETED";
                                            let mut input11 = String::new();

                                            println!("Enter Current class:");
                                            io::stdin().read_line(&mut input11).expect("Failed to read input");
                                            let _class:String = input11.trim().parse().expect("Invalid input");

                                            let _a_class:String = format!("Current Class: {}",_class);

                                            users_inputs.push(_waecval.to_string());
                                            users_inputs.push(_a_class.to_string());

                                        }
                        }

            } 
            else {
                  println!("That is all");
              }
    }  




//display data of all sibs

println!("\nSuccess! Displaying Users Inormation...\n");

for elements in users_inputs.iter() {
    println!("{}",elements);
}


}
