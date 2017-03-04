mod account;
mod transaction;
use account::Account;

extern crate argparse;
extern crate chrono;

use argparse::{ArgumentParser, Store, StoreTrue, Collect};
use std::fmt;

fn handle_error(msg: String) {
    println!("ERROR: {}!", msg);
}

enum ArgsError {
    NoArgumentsSpecified( String )
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ArgsError::NoArgumentsSpecified(ref msg) => return write!(f, "{}", msg),
        };
    }
}

struct Args {
    pub amount: Option<f64>,
    pub name: String
}

impl Args {
    fn from_string(s: String) -> Result<Args, ArgsError> {
        let parts: Vec<&str> = s.split(' ').collect();

        if parts.len() < 1 {
            return Err(ArgsError::NoArgumentsSpecified(String::from("No arguments given")));
        }

        let amount:Option<f64> = match parts[0].parse::<f64>() {
            Ok(v) => Some(v),
            Err(_) => None
        };

        match amount {
            Some(_) => return Ok(
                Args {
                    amount: amount,
                    name: parts[1..].join(" ")
                }),
            None => return Ok(Args { amount: amount, name: parts.join(" ") })
        };
    }
}

fn main() {
    let mut cmd = String::new();

    let cash = match Account::load(".my_account.finance".to_string()) {
        Ok(a) => a,
        Err(e) => { handle_error(e.to_string()); return; }
    };

    let mut accounts = vec![cash];
    let default_account = 0;
    let mut account = String::new();
    let mut can_overdraw = false;
    let mut account_index = default_account;
    let mut amount: Option<f64> = None;
    let mut description: String = String::from("No description");
    let mut str_args: Vec<String> = Vec::new();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("FINANCES");
        ap.refer(&mut cmd)
            .add_argument("COMMAND", Store, "Command to execute.")
            .required();
        ap.refer(&mut str_args)
            .add_argument("<args>", Collect, "The arguments of the command.");
        ap.refer(&mut account)
            .add_option(&["--account", "-a"], Store, "Account to operate on.");
        ap.refer(&mut description)
            .add_option(&["-d", "--description"], Store, "Description of the transaction or account.");
        ap.refer(&mut can_overdraw)
            .add_option(&["-o", "--overdraw"], StoreTrue, "Pass if the account can be overdrawn.");
        ap.parse_args_or_exit();
    }

    let args: Option<Args> = match Args::from_string(str_args.join(" ")) {
        Ok(a) => {
            amount = a.amount;
            Some(a)
        },
        Err(e) => { if cmd != "show" { handle_error(e.to_string()); return; }; None }
    };

    if account != "" {
        account_index = match accounts.iter().position(|x| x.name == account) {
            Some(i) => i,
            None => { println!("Unkown account, reverting to default!"); default_account }
        }
    }

    match cmd.as_ref() {
        "spent" => {
            match amount {
                Some(a) => {
                    match accounts[account_index].spent(a, description) {
                        Ok(_) => {},
                        Err(e) => { handle_error(e.to_string()); return; }
                    }
                },
                None => { handle_error("Command 'spent' requires an amount".to_string()); return; }
            };
        },
        "got" => {
            match amount {
                Some(a) => accounts[account_index].got(a, description),
                None => {
                    handle_error("Command 'got' requires an amount".to_string());
                    return;
                }
            };
        },
        "show" => {
            println!("{}", accounts[account_index]);
        },
        "new" => {
            match args {
                Some(a) => {
                    let account: Account = Account::create(a.name, description, can_overdraw);
                    match account.save() {
                        Ok(a) => { println!("Created account {}", a.name); },
                        Err(e) => { handle_error(e.to_string()); return; }
                    };
                    accounts.push(account);
                },
                None => { handle_error("Command 'new' needs at least a name specified".to_string()); return; }
            }
        },
        _ => {
            handle_error("Invalid command supplied".to_string());
            return;
        }
    };
    match accounts[account_index].save() {
        Ok(_) => {},
        Err(e) => { handle_error(e.to_string()); return; }
    }
}
