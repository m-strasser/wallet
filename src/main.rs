mod account;
mod transaction;
use account::Account;

extern crate argparse;
extern crate chrono;

use std::fs::{OpenOptions};
use std::io::{Write};
use argparse::{ArgumentParser, Store, Collect};

fn handle_error(msg: String) {
    println!("ERROR: {}!", msg);
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

fn main() {
    let mut expense = 0;
    let mut cmd = String::new();

    let cash = match Account::load(".my_account.finance".to_string()) {
        Ok(a) => a,
        Err(e) => { handle_error(e.to_string()); return; }
    };

    let mut accounts = vec![cash];
    let default_account = 0;
    let mut account = String::new();
    let mut account_index = default_account;

    let mut amounts: Vec<f64> = Vec::new();

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
        ap.refer(&mut account)
            .add_option(&["--account"], Store, "Account to operate on.");
        ap.parse_args_or_exit();
    }

    if account != "" {
        account_index = match accounts.iter().position(|x| x.name == account) {
            Some(i) => i,
            None => { println!("Unkown account, reverting to default!"); default_account }
        }
    }

    match cmd.as_ref() {
        "spent" => {
            for amount in amounts {
                match accounts[account_index].spent(amount, String::from("Stuff")) {
                    Ok(_) => {
                        match accounts[account_index].save() {
                            Ok(_) => {},
                            Err(e) => { handle_error(e.to_string()); return; }
                        }
                    },
                    Err(e) => { handle_error(e.to_string()); return; }
                }
            }
        },
        "got" => {
            got(amounts);
        },
        "show" => {
            println!("{}", accounts[account_index]);
        },
        _ => {
            handle_error("Invalid command supplied".to_string());
            return;
        }
    };
}
