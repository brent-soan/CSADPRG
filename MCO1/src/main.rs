use core::fmt;
use std::collections::HashMap;
use std::io;
use std::io::Write;

#[derive(Hash, Eq, PartialEq, Debug)]
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
    
    let mut exchange_rate: HashMap<Currency, f64> = HashMap::new();
    exchange_rate.insert(Currency::PHP, 1.0);
    exchange_rate.insert(Currency::USD, 0.017);
    exchange_rate.insert(Currency::JPY, 2.6);
    exchange_rate.insert(Currency::GBP, 0.013);
    exchange_rate.insert(Currency::EUR, 0.015);
    exchange_rate.insert(Currency::EUR, 0.12);

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
        } else if user_input == "2" && name != "" && balance < 1_000_000.0 {
            deposit(&name, &mut balance, &currency);
        } else if user_input == "3" && name != "" && balance > 0.0 {
            withdraw(&name, &mut balance, &currency);
        } else if user_input == "4" && name != "" {
            update_exchange_rate(&mut exchange_rate);
        } else if user_input == "5" && name != "" {

        } else if user_input == "6" && name != "" {

        } else if name == "" {
            println!("ERROR: Register first.");
        }else {
            println!("ERROR: Input not valid.");
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
            }
            
            *name = user_input;
        }
        
        is_finished = prompt();
        is_name_valid = false;
    }
}

fn deposit(name: &String, balance: &mut f64, currency: &Currency) {
    let mut is_finished: bool = false;
    let mut is_deposit_valid: bool = false;
    let mut user_input: String;
    let mut parsed_user_input: f64;

    while !is_finished {
        if !is_deposit_valid {
            println!("\nWithdraw Amount
Account Name: {name}
Current Balance: {balance}
Currency: {}", currency.to_str());
            user_input = input("Deposit Amount");
            
            if !user_input.parse::<f64>().is_ok() {
                println!("ERROR: Input not valid.");
                continue;
            }

            parsed_user_input = user_input.parse::<f64>().unwrap();

            if parsed_user_input <= 0.0 {
                println!("ERROR: Deposit must be greater than 0.");
                continue;
            } else if parsed_user_input + *balance > 1_000_000.0 {
                println!("ERROR: Balance must be less than or equal to 1,000,000 after depositing.");
                continue;
            }

            *balance += user_input.parse::<f64>().unwrap();
            println!("Updated Balance: {}", balance);
        }

        if *balance == 1_000_000.0 {
            break;
        }

        is_finished = prompt();
        is_deposit_valid = false;
    }
}

fn withdraw(name: &String, balance: &mut f64, currency: &Currency) {
    let mut is_finished: bool = false;
    let mut is_deposit_valid: bool = false;
    let mut user_input: String;
    let mut parsed_user_input: f64;

    while !is_finished {
        if !is_deposit_valid {
            println!("\nDeposit Amount
Account Name: {name}
Current Balance: {balance}
Currency: {}", currency.to_str());
            user_input = input("Deposit Amount");
            
            if !user_input.parse::<f64>().is_ok() {
                println!("ERROR: Input not valid.");
                continue;
            }

            parsed_user_input = user_input.parse::<f64>().unwrap();

            if parsed_user_input <= 0.0 {
                println!("ERROR: Withdraw must be greater than 0.");
                continue;
            } else if *balance - parsed_user_input < 0.0 {
                println!("ERROR: Balance must be at least 0 after withdrawing.");
                continue;
            }

            *balance -= user_input.parse::<f64>().unwrap();
            println!("Updated Balance: {}", balance);
        }

        if *balance == 0.0 {
            break;
        }

        is_finished = prompt();
        is_deposit_valid = false;
    }
}

fn update_exchange_rate(exchange_rate: &mut HashMap<Currency, f64>) {
    let mut is_finished: bool = false;
    let mut is_currency_valid: bool = false;
    let mut user_input: String;
    let mut currency: i8;

    while !is_finished {
        if !is_currency_valid {
            user_input = input("Record Exchange Rate
[1] United States Dollar (USD)
[2] Japanese Yen (JPY)
[3] British Pound Sterling (GBP)
[4] Euro (EUR)
[5] Chinese Yuan Renminni (CNY)
Select Foreign Currency");

            if !user_input.parse::<i8>().is_ok() {
                println!("ERROR: Input not valid.");
                continue;
            }

            currency = user_input.parse::<i8>().unwrap();

            if currency < 1 || currency > 5 {
                println!("ERROR: Input not valid.");
                continue;
            }
        }

        is_finished = prompt();
        is_currency_valid = false;
    }
}