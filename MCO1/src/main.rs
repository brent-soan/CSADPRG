use std::io;
use std::io::Write;

fn main() {
    let mut is_running = true;
    let mut input = String::new();

    println!("Welcome to MCO1 Banking and Currency App made with Rust!\n");

    // Main loop
    while is_running {
        print!("Main Menu
[0] Exit
[1] Register Account Name
[2] Deposit Amount
[3] Withdraw Amount
[4] Currency Exchange
[5] Record Exchange Rates
[6] Show Interest Amount
Select Transaction: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("ERROR: Input failed.");

        if input.trim() == "0" {
            is_running = false;
        } else {
            println!("ERROR: Incorrect input.\n");
        }

        input.clear();
    }

    print!("\nThank you and goodbye!");
}