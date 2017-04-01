/**
 * Handles functionality over multiple accounts.
 **/

use account::ACCOUNTS_FILE;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Error};
use account::Account;

pub fn print_overview(accounts: &Vec<Account>) {
    let mut overall_balance = 0.0;
    for account in accounts {
        println!("{}", account);
        overall_balance += account.balance;
    }
    println!("=====================================================");
    println!("{:.*}", 2, overall_balance);
}

pub fn print_account(name: String, accounts: &Vec<Account>) {
    let account_index = match (*accounts).iter().position(|x| x.name == name) {
        Some(i) => i,
        None => {
            println!("Unkown account, printing overview.");
            print_overview(&accounts);
            return;
        }
    };

    for transaction in accounts[account_index].transactions.iter() {
        println!("{}", transaction);
    }
}

pub fn load_accounts() -> Result<Vec<Account>, Error> {
    let mut accounts = Vec::new();

    let f = match OpenOptions::new().read(true).open(&ACCOUNTS_FILE) {
        Ok(f) => f,
        Err(e) => { return Err(e); }
    };
    let reader = BufReader::new(&f);

    for line in reader.lines() {
        match line {
            Ok(l) => {
                match Account::load(l) {
                    Ok(a) => accounts.push(a),
                    Err(e) => { return Err(e); }
                }
            },
            Err(e) => { return Err(e); }
        };
    }

    return Ok(accounts);
}
