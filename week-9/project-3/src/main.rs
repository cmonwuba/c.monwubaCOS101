use std::io::Write;
use std::io::Read;

fn main() {
    

    let arr1 = ["S/N. NAME OF COMMISSIONER","1. Aigbogun Alamba Daudu","2. Murtala Afeez Bendu","3. Okorocha Calistus Ogbona","4. Adewale Jimoh Akanbi","5. Osazuwa Faith  Etieye\n"];

    let arr2 = ["\nS/N. MINISTRY","1. Internal affairs","2. Justice","3. Defense","4. Power and Steel","5. Petroleum\n"];

    let arr3 = ["\nS/N. Geo-political Zone","1. South West","2. North East","3. South South","4. South West","5. South East\n"];

    //Converting array to string

    let arr1str : String = arr1.join(" \n ");
    let arr2str : String = arr2.join(" \n ");
    let arr3str : String = arr3.join(" \n ");



    //Writing array to file
 
    let mut file = std::fs::File::create("data1.txt").expect("create failed");
    file.write_all(arr1str.as_bytes()).expect("write failed");

    let mut file = std::fs::File::create("data2.txt").expect("create failed");
    file.write_all(arr2str.as_bytes()).expect("write failed");

    let mut file = std::fs::File::create("data3.txt").expect("create failed");
    file.write_all(arr3str.as_bytes()).expect("write failed");


    // Merging the three files via reading and writing into variable

    let mut file = std::fs::File::open("data1.txt").unwrap();
    let mut content1 = String::new();
    file.read_to_string(&mut content1).unwrap();

    let mut file = std::fs::File::open("data2.txt").unwrap();
    let mut content2 = String::new();
    file.read_to_string(&mut content2).unwrap();

    let mut file = std::fs::File::open("data3.txt").unwrap();
    let mut content3 = String::new();
    file.read_to_string(&mut content3).unwrap();


    //Creating a new file for the merge data

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all(content1.as_bytes()).expect("write failed");
    file.write_all(content2.as_bytes()).expect("write failed");
    file.write_all(content3.as_bytes()).expect("write failed");


    //displaying merged file

    println!("File Merge Successfully!\n");

    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();


    println!("{}",contents);


}
