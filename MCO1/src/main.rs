use std::io;
use std::io::Write;

enum Currency {
    PHP,
    USD,
    JPY,
    GBP,
    EUR,
    CNY
}

fn main() {
    let mut is_running = true;
    let mut user_input = String::new();

    let mut name: String = String::new();
    let mut currency: Currency = Currency::PHP;
    let mut balance: f64 = 0.0;

    println!("Welcome to MCO1 Banking and Currency App made with Rust!");

    // Main loop
    while is_running {
        user_input = input("\nMain Menu
[0] Exit
[1] Register Account Name
[2] Deposit Amount
[3] Withdraw Amount
[4] Currency Exchange
[5] Record Exchange Rates
[6] Show Interest Amount
Select Transaction");
        
        if user_input.trim() == "0" {
            is_running = false;
        } else if user_input.trim() == "1" {
            register(&name);
        } else if user_input.trim() == "2" {

        } else if user_input.trim() == "3" {

        } else if user_input.trim() == "4" {

        } else if user_input.trim() == "5" {

        } else if user_input.trim() == "6" {

        } else {
            println!("ERROR: Incorrect user_input.");
        }

        user_input.clear();
    }

    print!("\nThank you and goodbye!");
}

fn input(prompt: &str) -> String {
    let mut user_input = String::new();
    
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input).expect("ERROR: Input failed.");

    user_input
}

fn register(name: &String) {
    //name = input("Register Account Name
//Account Name");
    input("Back to the Main Menu (Y/N)");
}