mod interval;
mod account;
mod transaction;
mod dateutils;
mod category;
mod accountmanager;
mod arguments;
mod commands;

#[cfg(test)]
mod test_recurring;

use account::Account;
use accountmanager::print_overview;
use accountmanager::print_account;
use accountmanager::load_accounts;
use arguments::handle_args;
use commands::{New, Got, Set, Spent, Command};
use std::fs::{File};
use std::io::{BufReader, BufRead};
use std::path::Path;

extern crate argparse;
extern crate chrono;

fn handle_error(msg: String) {
    println!("ERROR: {}!", msg);
}

fn main() {
    let default_account = 0;
    let mut account_index = default_account;
    let args = handle_args();
    let mut amount: Option<f64> = None;

    let mut accounts: Vec<Account> = match load_accounts() {
        Ok(a) => a,
        Err(e) => { handle_error(e.to_string()); return; }
    };

    if accounts.len() == 0 && args.cmd != "new" {
        println!("There are no accounts yet.");
        println!("You can create one with the 'new' command.");
        return;
    }

    // match args.collected {
    //     Some(ref c) => { amount = (&c.amount).clone(); },
    //     None => if args.cmd != "show" {
    //         handle_error("You need to specify an amount".to_string());
    //         return;
    //     }
    // }

    match args.account {
        Some(ref a) => {
            account_index = match accounts.iter().position(|x| x.name == a.clone()) {
                Some(i) => i,
                None => { println!("Unkown account, reverting to default!"); default_account }
            };
        },
        None => {
            println!("Unkown account, reverting to default!");
            account_index = default_account;
        }
    }

    let new = New {};
    let spent = Spent {};
    let got = Got {};
    let set = Set {};
    // let show = Show { name: "show".to_string(), accounts: &accounts };

    match args.cmd.as_ref() {
        "new" => {
            match new.execute(None, &args) {
                Ok(x) => {
                    match x {
                        Some(a) => { accounts.push(a); },
                        None => {
                            handle_error("An unexpected error occurred".to_string());
                            return;
                        }
                    }
                },
                Err(e) => {
                    handle_error(e.to_string());
                    return;
                }
            }
        },
        "spent" => {
            match spent.execute(Some(&mut accounts[account_index]), &args) {
                Ok(_) => {},
                Err(e) => {
                    handle_error(e.to_string());
                    return;
                }
            }
        },
        "got" => {
            match got.execute(Some(&mut accounts[account_index]), &args) {
                Ok(_) => {},
                Err(e) => {
                    handle_error(e.to_string());
                    return;
                }
            }
        },
        "set" => {
            match set.execute(Some(&mut accounts[account_index]), &args) {
                Ok(_) => {},
                Err(e) => {
                    handle_error(e.to_string());
                    return;
                }
            }
        },
        "show" => {
            match args.account {
                Some(a) => { print_account(a, args.time_frame, &accounts); },
                None => { print_overview(&accounts); }
            }
        },
        _ => {
            handle_error("Invalid command supplied".to_string());
            return;
        }
    }

    match accounts[account_index].save() {
        Ok(_) => {},
        Err(e) => { handle_error(e.to_string()); return; }
    }
}
