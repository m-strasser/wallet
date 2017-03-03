extern crate argparse;

use std::io;
use std::fs::{OpenOptions};
use std::io::{Write, BufReader, BufRead};
use argparse::{ArgumentParser, Store, Collect};

fn handle_error(msg: String) {
    println!("ERROR: {}!", msg);
}

fn spent(amounts: Vec<f64>) {
    if amounts.len() < 1 {
        handle_error("At least one amount hast to be specified".to_string());
        return;
    }
    println!("Adding new expense(s) {:?}", amounts);
    store(amounts.into_iter().map(|x| x * (-1.0)).collect());
}

fn got(amounts: Vec<f64>) {
    if amounts.len() < 1 {
        handle_error("At least one amount hast to be specified".to_string());
        return;
    }
    println!("Adding new income(s) {:?}", amounts);
    store(amounts);
}

fn store(amounts: Vec<f64>) {
    let mut f = match OpenOptions::new().append(true).open("expenses.txt") {
        Ok(f) => f,
        Err(e) => { handle_error(e.to_string()); return; }
    };
    for amount in &amounts {
        match write!(f, "{:.*}\n", 2, amount) {
            Ok(_) => {}
            Err(e) => { handle_error(e.to_string()); return; }
        };
    }
}

fn restore_account(path: String) -> Result<Account, io::Error> {
    let parts: Vec<&str> = path.split(':').collect();
    if (parts.len()) < 3 {
        return Err(io::Error::new(io::ErrorKind::Other,
                              "Stored account needs to contain 3 parameters!"));
    }
    let name: String = parts[0].to_string();
    let path: String = parts[1].to_string();
    let can_overdraw: bool = match parts[2].parse::<bool>() {
        Ok(val) => val,
        Err(_) => return Err(io::Error::new(io::ErrorKind::Other,
                                            "Third account value needs to be bool!"))
    };

    let mut transactions: Vec<f64> = Vec::new();
    let f = match OpenOptions::new().read(true).open(&path){
        Ok(f) => f,
        Err(e) => return Err(e)
    };
    let reader = BufReader::new(&f);

    for res in reader.lines() {
        match res {
            Ok(line) => {
                match line.parse::<f64>() {
                    Ok(amount) => transactions.push(amount),
                    Err(_) => return Err(io::Error::new(io::ErrorKind::Other,
                                                        "Transactions must be convertable to float!"))
                };
            },
            Err(e) => return Err(e)
        };
    }

    Ok(Account {
        name: name,
        filepath: path,
        balance: 0.0,
        can_overdraw: can_overdraw,
        transactions: Box::<Vec<f64>>::new(transactions)
    })
}

fn restore(path: String) -> Result<Vec<Account>, io::Error> {
    let mut accounts: Vec<Account> = Vec::new();
    let mut account_paths: Vec<Result<String, io::Error>> = Vec::new();

    let f = match OpenOptions::new().read(true).open(path) {
        Ok(f) => f,
        Err(e) => return Err(e)
    };
    let reader = BufReader::new(&f);

    for res in reader.lines() {
        account_paths.push(res);
    }

    for account_path in account_paths {
        match account_path {
            Ok(line) => {
                match restore_account(line) {
                    Ok(acc) => accounts.push(acc),
                    Err(e) => return Err(e)
                };
            },
            Err(e) => return Err(e)
        }
    }

    return Ok(accounts);
}

#[derive (Debug)]
struct Account {
    name: String,
    filepath: String,
    balance: f64,
    can_overdraw: bool,
    transactions: Box<Vec<f64>>
}

fn main() {
    let mut expense = 0;
    let mut cmd = String::new();
    let mut amounts: Vec<f64> = Vec::new();
    let accounts: Vec<Account> = match restore(".accounts".to_string()) {
        Ok(accs) => accs,
        Err(e) => { handle_error(e.to_string()); return; }
    };

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("FINANCES");
        ap.refer(&mut cmd)
            .add_argument("COMMAND", Store, "Command to execute.")
            .required();
        ap.refer(&mut amounts)
            .add_argument("AMOUNT", Collect, "Money spent.");
        ap.refer(&mut expense)
            .add_option(&["--expense"], Store, "Amount spent.");
        ap.parse_args_or_exit();
    }

    match cmd.as_ref() {
        "spent" => {
            spent(amounts);
        },
        "got" => {
            got(amounts);
        },
        "show" => {
            println!("{:?}", accounts);
        },
        _ => {
            handle_error("Invalid command supplied".to_string());
            return;
        }
    };
}
