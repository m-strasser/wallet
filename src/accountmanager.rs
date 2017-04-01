/**
 * Handles functionality over multiple accounts.
 **/

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
