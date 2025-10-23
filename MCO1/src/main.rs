use core::fmt;
use std::io;
use std::io::Write;

enum Currency {
    PHP, // Philippine Peso 
    USD, // United States Dollar
    JPY, // Japanese Yen
    GBP, // British Pound Sterling
    EUR, // Euro
    CNY  // Chinese Yuan Renmini
}

impl Currency {
    fn to_str(&self) -> &'static str {
        match self {
            Currency::PHP => "PHP",
            Currency::USD => "USD",
            Currency::JPY => "JPY",
            Currency::GBP => "GBP",
            Currency::EUR => "EUR",
            Currency::CNY => "CNY",
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn main() {
    let mut is_finished = false;  
    let mut user_input;

    let mut name: String = String::new();
    let mut currency: Currency = Currency::PHP;
    let mut balance: f64 = 0.0;

    println!("Welcome to MCO1 Banking and Currency App made with Rust!");

    // Main loop
    while !is_finished {
        user_input = input("\nMain Menu
[0] Exit
[1] Register Account Name
[2] Deposit Amount
[3] Withdraw Amount
[4] Currency Exchange
[5] Record Exchange Rates
[6] Show Interest Amount
Select Transaction");
        
        if user_input == "0" {
            is_finished = true;
        } else if user_input == "1" {
            register(&mut name);
        } else if user_input == "2" && name != "" {
            deposit(&name, &mut balance, &currency);
        } else if user_input == "3" && name != "" {

        } else if user_input == "4" && name != "" {

        } else if user_input == "5" && name != "" {

        } else if user_input == "6" && name != "" {

        } else if name == "" {
            println!("ERROR: Register first.");
        }else {
            println!("ERROR: Incorrect transaction input.");
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

    user_input.trim().to_string()
}

fn prompt() -> bool {
    let mut user_input: String;

    loop {
        user_input = input("\nBack to the Main Menu (Y/N)");

        if user_input == "Y" {
            return true;
        } else if user_input == "N" {
            return false;
        }

        println!("ERROR: Enter Y or N only.");
    }
}

fn register(name: &mut String) {
    let mut is_finished: bool = false;
    let mut is_name_valid: bool = false;
    let mut user_input: String;

    while !is_finished {
        if !is_name_valid {
            user_input = input("\nRegister Account Name
Account Name");

            for c in user_input.chars() {
                if !c.is_alphabetic() && c != ' ' {
                    is_name_valid = false;
                    break;
                } else if !is_name_valid {
                    is_name_valid = true;
                }
            }

            if !is_name_valid {
                println!("ERROR: Name not valid. Alphabets and spaces are allowed.");
                continue;
            } else {
                *name = user_input;
            }
        }
        
        is_finished = prompt();
        is_name_valid = false;
    }
}

fn deposit(name: &String, balance: &mut f64, currency: &Currency) {
    let mut is_finished: bool = false;
    let mut is_balance_valid: bool = false;
    let mut user_input: String;

    while !is_finished {
        if !is_balance_valid {
            println!("Deposit Amount
Account Name: {name}
Current Balance: {balance}
Currency: {}", currency.to_str());
            user_input = input("Deposit Amount");
        }

        is_finished = prompt();
        is_balance_valid = false;
    }
}