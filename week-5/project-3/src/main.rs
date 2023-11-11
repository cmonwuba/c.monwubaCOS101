use std::io;

fn main() {
  
  let p = "Poundo yam/Edinkaiko Soup";
  let f = "Fried Rice & Chicken";
  let a = "Amala & Ewedu Soup";
  let e = "Eba & Egusi Soup";
  let w = "White Rice & Stew";
loop {
    println!("                  Menu                                   Price                  

             P = Poundo Yam/Edinkaiko Soup            NGN 3,200   
             F = Fried Rice & Chicken                 NGN 3,000     
             A = Amala & Ewedu Soup                   NGN 2,500 
             E = Eba & Egusi Soup                     NGN 2,000
             W = White Rice & Stew                    NGN 2,500 
");


   let mut food = String::new();
  println!(" What would you like to eat : ");
   io::stdin().read_line(&mut food).expect("Failed to read input");
   let food = food.trim().to_lowercase();

   if food == "p" || food == "Poundo Yam/Edinkaiko Soup"
   { println!(" How many portions of {} would you like to buy",p);}

   else if food == "f" || food == "Fried Rice & Chicken"
   {println!("How many portions of {} would you like to buy",f);}

   else if food == "a" || food == "Amala & Ewedu Soup"
   {println!("How many portions of {} would you like to buy",a);}

   else if food == "e" || food == "Eba & Egusi Soup"
   {println!("How many portions of {} would you like to buy",e);}

   else if food == "w" || food == "White rice & Stew"
   {println!("How many portions of {} would you like to buy",w);}

   else {
    println!("Not an item on the menu. Try again");continue;
}

   let mut quantity = String::new();
   io::stdin().read_line(&mut quantity).expect("Failed to read input");
   let quantity:f32 = quantity.trim().parse().expect("Failed to read input");

   if food == "p" || food == "Poundo Yam/Edinkaiko Soup"
   {
       let calc:f32 = 3200.0 * quantity;
       if calc > 10000.0 {
            let calc:f32 = calc * 0.95;
            println!("You qualify for the discount and your price is: N{}", calc);break;
        }
        else {

            println!("Your balance is N{}", calc);break;
        } }
        else if food == "f" || food == "Fried Rice & Chicken"
        {
            let calc1:f32 = 3000.0 * quantity;
            if calc1 > 10000.0 {
                let calc1:f32 = calc1 * 0.95;
                println!("You qualify for the discount and your price is: N{}", calc1);break;
        }
        else {

            println!("Your balance is N{}", calc1); break;
        }}

        else if food == "a" || food == "Amala & Ewedu Soup "
        {
            let calc2:f32 = 2500.0 * quantity;
            if calc2 > 10000.0 {
                let calc2:f32 = calc2 * 0.95;
                println!("You qualify for the discount and your price is: N{}", calc2);break;
        }
        else {

            println!("Your balance is N{}", calc2); break;
        }}

        else if food == "e" || food == "Eba & Egusi Soup"
        {
            let calc3:f32 = 2000.0 * quantity;
            if calc3 > 10000.0 {
                let calc3:f32 = calc3 * 0.95;
                println!("You qualify for the discount and your price is: N{}", calc3);break;
        }
        else {

            println!("Your balance is N{}", calc3); break;
        }}

        else if food == "w" || food == "White Rice & Stew"
        {
            let calc4:f32 = 2500.0 * quantity;
            if calc4 > 10000.0 {
                let calc1:f32 = calc4 * 0.95;
                println!("You qualify for the discount and your price is: N{}", calc4);break;
        }
        else {

            println!("Your balance is N{}", calc4); break;
        }}

    }}













   

