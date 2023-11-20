use std::io;

fn main() {
    let mut researcher_num = String::new();
    let mut researcher_name = String::new();
    let mut number_of_Papers = String::new();


    println!("Hello candidate, what's your number?");
    io::stdin().read_line(&mut researcher_num).expect("Failed to read input");
    let researcher_num:i32 = researcher_num.trim().parse().expect("Failed to read input");

    if researcher_num > 500{
        println!("Sorry, only the first 500 can participate");
        return;
     }
     
     else{
        println!("What is your name?");
        io::stdin().read_line(&mut researcher_name).expect("Failed to read input");
    let researcher_name = researcher_name.trim();
}
println!();

println!("How many papers have you published?",);
io::stdin().read_line(&mut number_of_Papers).expect("Failed to read input");
let number_of_Papers:i32 = number_of_Papers.trim().parse().expect("Failed to read input");

if number_of_Papers >=3 && number_of_Papers <5{
    println!("Your incentive is N500,000");
}

else if number_of_Papers >5 && number_of_Papers <10{
    println!("Your incentive is N800,000");
}

else if number_of_Papers >10{
    println!("Your incentive is 1,000,000");
}

else if number_of_Papers <3{
    println!("Your incentive is N100,000");
}


}
