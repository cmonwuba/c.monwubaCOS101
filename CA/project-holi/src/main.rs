use std::io;
use std::io::Write;

// Define a struct to help represent an organization
struct Organization {
    name: String,
    date_founded: i32,
    assets: f32,
    liabilities: i32,
    lev_percent: f32,
}


// Function to check if a username is valid
fn is_valid_username(username: &str) -> bool {
    username.chars().all(|c| c.is_ascii_alphanumeric())
}

// Function to check if a password is valid
fn is_valid_password(password: &str) -> bool {
    println!("Checking valid password {}", password);
    if password.len() < 3 || password.len() > 8 {
        return false;
    }

    for ch in password.chars() {
        if !ch.is_lowercase() && !ch.is_ascii_alphanumeric() {
            return false;
        }
    }
    return true;
}

fn main() {

    //Get user input for username and password

    println!("Enter your username (less than 4 characters):");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");
    let username = username.trim();

    println!("Enter your password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read line");
    let password = password.trim();

    //Check if username and password are valid

    if username.len() < 4 && is_valid_username(username) && is_valid_password(password) {
        println!("Access granted! Welcome, {}!", username);
    } else {
        println!("Invalid username or password. Please try again.");
    }

    
    // Create a file to store organization data
    let mut file = std::fs::File::create("organization.txt").expect("create failed");
    

    // Welcome Message
    let comp = "Welcome to Nigerian Companies Data Management System";
    
    //Create a vector of organizations
    let mut organizations = vec![

        Organization {
            name: String::from("Cadbury Nigeria Plc"),
            date_founded: 1965,
            assets: 15_000_000.0,
            liabilities: 5_500_000,
            lev_percent: 0.0063,
        },

        Organization {
            name: String::from("Champions Breweries Plc"),
            date_founded: 1974,
            assets: 25_000_000.0,
            liabilities: 8_000_000,
            lev_percent: 0.0068,
        },

        Organization {
            name: String::from("Dangote Sugar Refinery Plc"),
            date_founded: 1970,
            assets: 18_000_000.0,
            liabilities: 10_000_000,
            lev_percent: 0.00444,
        },

        Organization {
            name: String::from("Flour Mills Nigeria Plc"),
            date_founded: 1960,
            assets: 32_000_000.0,
            liabilities: 4_000_000,
            lev_percent: 0.00875,
        },

        Organization {
            name: String::from("Nestle Nigeria Plc"),
            date_founded: 1961,
            assets: 8_000_000.0,
            liabilities: 1_500_000,
            lev_percent: 0.00813,
        },

        Organization {
            name: String::from("Unilever Nigeria Plc"),
            date_founded: 1923,
            assets: 37_000_000.0,
            liabilities: 11_000_000,
            lev_percent: 0.00703,
        },

        Organization {
            name: String::from("Honeywell Nigeria Plc"),
            date_founded: 1906,
            assets: 34_000_000.0,
            liabilities: 9_000_000,
            lev_percent: 0.00735,
        },

        Organization {
            name: String::from("Nigerian Breweries Plc"),
            date_founded: 1946,
            assets: 30_000_000.0,
            liabilities: 12_000_000,
            lev_percent: 0.00600
        },
    ];


    //Write organization data to the file
    for organization in &organizations {
        writeln!(
            &mut file,
            "Name: {}\nDate Founded: {}\nAssets: N{}\nLiabilities: N{}\nLeverage Percentage: {}%\n",
            organization.name,
            organization.date_founded,
            organization.assets,
            organization.liabilities,
            organization.lev_percent
        )
        .expect("Error: Not able to write to file");
    }

    println!("Organization has being written to organization.txt successfully");


    // Create a file to store leverage multiples
    let mut file_2 = std::fs::File::create("leverage_multiples.txt").expect("create failed");



    // Write leverage multiples for organization with assets/shares greater than twenty million to the file
    for organization in &organizations {
        if organization.assets > 20_000_000.0 {
            file_2.write_all("Company Name: ".as_bytes()).expect("Failed to write to file");
            file_2.write_all(organization.name.as_bytes()).expect("Failed to write to file");
            file_2.write_all(b"\n").expect("Failed to write to file");

            file_2.write_all("Assets: ".as_bytes()).expect("Failed to write to file");
            file_2.write_all(organization.assets.to_string().as_bytes()).expect("Failed to write to file");
            file_2.write_all(b"\n").expect("Failed to write to file");

            file_2.write_all("Leverage Percentage: ".as_bytes()).expect("Failed to write to file");
            file_2.write_all(
                organization
                    .lev_percent
                    .to_string()
                    .chars()
                    .map(|c| if c == '.' { ',' } else { c })
                    .collect::<String>()
                    .as_bytes(),
            )
            .expect("Failed to write to file");

            file_2.write_all(b"%\n\n").expect("Failed to write to file");
        }
    }

    println!("Leverage multiples for all companies have been written to leverage_multiples.txt successfully");

    // Create a file to store calculated liabilities leverage
    let mut file_3 = std::fs::File::create("liabilities_leverage.txt").expect("create failed");


    // Write calculated liabilities leverage for organizations with liablities less than ten million to the file
    for organization in &organizations {
        if organization.liabilities < 10_000_000 {

            let lia_leverage = organization.lev_percent * 0.05;

            file_3.write_all("Company Name: ".as_bytes()).expect("Failed to write to file");
            file_3.write_all(organization.name.as_bytes()).expect("Failed to write to file");
            file_3.write_all(b"\n").expect("Failed to write to file");

            file_3.write_all("Calculated Liabilities Leverage: ".as_bytes()).expect("Failed to write to file");
            file_3.write_all(organization.liabilities.to_string().as_bytes()).expect("Failed to write to file");
            file_3.write_all(b"\n\n").expect("Failed to write to file");

        }
    }

    println!("Calculate liability percentage leverage written to liabilities_leverage.txt successfully");

}
    






    





    












   