mod interval;
mod account;
mod transaction;
mod dateutils;
mod category;
mod accountmanager;
mod arguments;

#[cfg(test)]
mod test_recurring;

use account::Account;
use accountmanager::print_overview;
use accountmanager::print_account;
use accountmanager::load_accounts;
use arguments::handle_args;

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

    match args.collected {
        Some(ref c) => { amount = (&c.amount).clone(); },
        None => if args.cmd != "show" {
            handle_error("You need to specify an amount".to_string());
            return;
        }
    }

    match args.account {
        Some(a) => {
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

    match args.cmd.as_ref() {
        "spent" => {
            match amount {
                Some(a) => {
                    match accounts[account_index].spent(a, args.description) {
                        Ok(_) => {},
                        Err(e) => { handle_error(e.to_string()); return; }
                    }
                },
                None => { handle_error("Command 'spent' requires an amount".to_string()); return; }
            };
        },
        "got" => {
            match amount {
                Some(a) => accounts[account_index].got(a, args.description),
                None => {
                    handle_error("Command 'got' requires an amount".to_string());
                    return;
                }
            };
        },
        "show" => {
            match args.collected {
                Some(a) => { print_account(a.name, &accounts); },
                None => { print_overview(&accounts); }
            }
        },
        "new" => {
            match args.collected {
                Some(a) => {
                    let account: Account = match Account::create(a.name, args.description, args.can_overdraw) {
                        Ok(a) => a,
                        Err(e) => { handle_error(e.to_string()); return; }
                    };
                    match account.save() {
                        Ok(a) => { println!("Created account {}", a.name); },
                        Err(e) => { handle_error(e.to_string()); return; }
                    };
                    accounts.push(account);
                },
                None => { handle_error("Command 'new' needs at least a name specified".to_string()); return; }
            }
        },
        "set" => {
          match amount {
            Some(a) => {
              accounts[account_index].set(a, args.description);
            },
            None => { handle_error("Command set needs at least an amount specified".to_string()); return; }
          }
        },
        "category" => {
            // match str_args[0].as_ref() {
            //     "add" => {
            //         if str_args.len() < 2 {
            //             handle_error("category add needs at least a name specified".to_string());
            //             return;
            //         }
            //     }
            //     _ => {}
            // }
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
