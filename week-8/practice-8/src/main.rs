fn main() {
    // initialize a mutable tuple
    let mut mountain_height = ("Everest", 8848, "Fishtail", 6993);

    println!("Original tuple = {:?}", mountain_height);

    // change 3rd and 4th element of a mutable tuple
    mountain_height.2 = "Lhotse";
    mountain_height.3 = 8516;

    println!("Changed tuple = {:?}", mountain_height);
}