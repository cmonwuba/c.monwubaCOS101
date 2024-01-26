use std::io;

struct Laptop {
        name: String,
        quantity: i32,
        amount: i32,
        price: i32
}

impl Laptop {
    fn calc(&self) -> i32 {
        self.amount * self.price

    }
}

impl Laptop {
    fn leftover(&self) -> i32 {
        self.quantity - self.amount

    }
}


fn main() {

    let mut input = String::new();
    println!("How many products do you want?(This applies to all)");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let x:i32 = input.trim().parse().expect("Invalid input");
    
    let hp = Laptop {
        name:String::from("Hewlett Packard"),
        quantity:10,
        amount:x,
        price:650_000
    };
    let ibm = Laptop {
        name:String::from("IBM"),
        quantity:6,
        amount:x,
        price:755_000
    };
    let toshiba = Laptop {
        name:String::from("Toshiba"),
        quantity:10,
        amount:x,
        price:550_000
    };
    let dell = Laptop {
        name:String::from("Dell"),
        quantity:4,
        amount:x,
        price:850_000
    };

    let sum = hp.calc() + ibm.calc() + toshiba.calc() + dell.calc();

    println!("\nProcessed Succesfully!\n");

    display(hp);
    display(ibm);
    display(toshiba);
    display(dell);

    println!("Your Sum Total is: N{}",sum);


}

fn display(pc: Laptop) {

    println!("Name: {} \nRequested Amount: {} \nEstimated Price: {} \nProucts Remaining: {}\n",pc.name,pc.amount,pc.calc(),pc.leftover());

}