extern crate argparse;

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
    store(amounts);
}

fn got(amounts: Vec<f64>) {
    if amounts.len() < 1 {
        handle_error("At least one amount hast to be specified".to_string());
        return;
    }
    println!("Adding new income(s) {:?}", amounts);
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

fn restore(path: String) -> Vec<f64> {
    let mut amounts: Vec<f64> = Vec::new();

    let f = match OpenOptions::new().read(true).open(path) {
        Ok(f) => f,
        Err(e) => { handle_error(e.to_string()); return amounts; }
    };
    let reader = BufReader::new(&f);

    for res in reader.lines() {
        match res {
            Ok(l) => {
                match l.parse::<f64>() {
                    Ok(amount) => amounts.push(amount),
                    Err(e) => { handle_error(e.to_string()); return amounts; }
                };
            },
            Err(e) => { handle_error(e.to_string()); return amounts; }
        }
    }

    return amounts;
}

fn main() {
    let mut expense = 0;
    let mut cmd = String::new();
    let mut amounts: Vec<f64> = Vec::new();

    println!("{:?}", restore("expenses.txt".to_string()));
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
        _ => {
            handle_error("Invalid command supplied".to_string());
            return;
        }
    };
}
