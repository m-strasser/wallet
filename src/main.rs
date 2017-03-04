mod account;
mod transaction;
use account::Account;

extern crate argparse;
extern crate chrono;

use argparse::{ArgumentParser, Store};

fn handle_error(msg: String) {
    println!("ERROR: {}!", msg);
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
    let mut account_index = default_account;
    let mut amount: f64 = 0.0;
    let mut description: String = String::from("No description");

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("FINANCES");
        ap.refer(&mut cmd)
            .add_argument("COMMAND", Store, "Command to execute.")
            .required();
        ap.refer(&mut amount)
            .add_argument("AMOUNT", Store, "The amount of money.");
        ap.refer(&mut account)
            .add_option(&["--account", "-a"], Store, "Account to operate on.");
        ap.refer(&mut description)
            .add_option(&["-d", "--description"], Store, "Description of the transaction.");
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
            match accounts[account_index].spent(amount, description) {
                Ok(_) => {},
                Err(e) => { handle_error(e.to_string()); return; }
            }
        },
        "got" => {
            accounts[account_index].got(amount, description);
        },
        "show" => {
            println!("{}", accounts[account_index]);
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
