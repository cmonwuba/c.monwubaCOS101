use std::io::Write;

fn main() {
    
let mut file = std::fs::File::create("Sims.txt").expect("create failed");

let stud = "Welcome to Student Information Management System";

let student_name:Vec<&str> = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
let matric_no:Vec<&str> = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
let dept:Vec<&str> = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical",];
let level:Vec<i32> = vec![300,100,200,200,100];

let mut person: Vec<(&str, &str, &str, i32)> = Vec::new();

for i in 0..student_name.len() {
    let person_info = (student_name[i],matric_no[i],dept[i],level[i]);
    person.push(person_info);
}

let header = ["Student name", "matric_no", "Department", "Level"];

file.write_all(b"\t\t").expect("Failed to write to file");
file.write_all(stud.as_bytes()).expect("Failed to write to file");
file.write_all(b"\n\n").expect("Failed to write to file");

for index in &header {
    file.write_all(index.as_bytes()).expect("write failed");
    file.write_all(b"\t").expect("write failed");
}


for index in &person {
    file.write_all(b"\n").expect("Failed to write to file");
    file.write_all(index.0.as_bytes()).expect("Failed to write to file");
    file.write_all(b"\t").expect("Failed to write to file");

    file.write_all(index.1.as_bytes()).expect("Failed to write to file");
    file.write_all(b"\t").expect("Failed to write to file");

    file.write_all(index.2.as_bytes()).expect("Failed to write to file");
    file.write_all(b"\t").expect("Failed to write to file");

    file.write_all(index.3.to_string().as_bytes()).expect("Failed to write to file");
    file.write_all(b"\n").expect("Failed to write to file");
}

}













    






    
