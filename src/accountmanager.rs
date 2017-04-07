/**
 * Handles functionality over multiple accounts.
 **/

use chrono::prelude::{UTC};
use chrono::{Duration};
use transaction::Transaction;
use account::{ACCOUNTS_FILE, BASE_PATH};
use std::env::home_dir;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Error, ErrorKind};
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

pub fn print_account(name: String, time_frame: Option<i64>,
                     accounts: &Vec<Account>) {
    let account_index = match (*accounts).iter().position(|x| x.name == name) {
        Some(i) => i,
        None => {
            println!("Unkown account, printing overview.");
            print_overview(&accounts);
            return;
        }
    };
    let mut expenses = 0.0;
    let mut income = 0.0;
    let mut ctf = false;
    let tf = match time_frame {
        Some(t) => { ctf = true; t},
        None => -1
    };

    for transaction in accounts[account_index].transactions.iter() {
        if !ctf || (UTC::now().date() - Duration::days(tf)) < transaction.date.date() {
            println!("{}", transaction);
            if transaction.amount > 0.0 {
                income += transaction.amount;
            } else {
                expenses += transaction.amount;
            }
        }
    }
    println!("============================================================");
    println!("Spent: {:.*}", 2, expenses);
    println!("Got: {:.*}", 2, income);
}

pub fn load_accounts() -> Result<Vec<Account>, Error> {
    let mut accounts = Vec::new();
    let home_dir = match home_dir() {
        Some(p) => p,
        None => {
            return Err(Error::new(ErrorKind::Other,
                "Could not get home directory"
            ));
        }
    };

    let f = match OpenOptions::new().read(true).open(
        home_dir.join(&BASE_PATH).join(&ACCOUNTS_FILE)) {
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
