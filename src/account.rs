use transaction::Transaction;
use interval::Interval;

use std::rc::Rc;
use std::io;
use std::fmt;
use std::fs::{OpenOptions};
use std::io::{Write, BufReader, BufRead};
use chrono::prelude::{UTC, DateTime};
use chrono::{Duration, Datelike};

#[derive (Debug)]
pub enum AccountError {
    NoOverdraw( String ),
    FileError( String )
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AccountError::NoOverdraw(ref msg) => return write!(f, "{}", msg),
            &AccountError::FileError(ref msg) => return write!(f, "{}", msg),
        };
    }
}

pub static ACCOUNTS_FILE: &'static str = ".accounts.finance";

#[derive (Debug)]
pub struct Account {
    pub name: String,
    description: String,
    filepath: String,
    pub balance: f64,
    can_overdraw: bool,
    pub transactions: Box<Vec<Rc<Transaction>>>
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({}): {}", self.name, self.balance, self.description)
    }
}

impl Account {
    pub fn new(n: String, d: String, fp:String, b: f64, o: bool, ts: Box<Vec<Rc<Transaction>>>)
        -> Account {
        Account {
            name: n,
            description: d,
            filepath: fp,
            balance: b,
            can_overdraw: o,
            transactions: ts
        }
    }

    pub fn create(name: String, description: String, can_overdraw: bool)
        -> Result<Account, AccountError> {
        let filepath = format!(".{}.finance", name.replace(" ", "_").to_lowercase()).to_string();

        let mut f = match OpenOptions::new().append(true).create(true).open(&ACCOUNTS_FILE) {
            Ok(f) => f,
            Err(e) => return Err(AccountError::FileError(e.to_string()))
        };

        match write!(f, "{}\n", filepath) {
            Ok(_) => {},
            Err(e) => return Err(AccountError::FileError(e.to_string()))
        }

        Ok(Account {
            name: name,
            description: description,
            filepath: filepath,
            balance: 0.0,
            can_overdraw: can_overdraw,
            transactions: Box::<Vec<Rc<Transaction>>>::new(Vec::new())
        })
    }

    pub fn load(path: String) -> Result<Account, io::Error> {
        let f = match OpenOptions::new().read(true).open(&path) {
            Ok(f) => f,
            Err(e) => return Err(e)
        };

        let mut reader = BufReader::new(&f);
        // First line is the definition of the account
        let mut account_def = String::new();
        match reader.read_line(&mut account_def) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }

        let parts: Vec<&str> = account_def.split(';').collect();
        if (parts.len()) < 3 {
            return Err(
                io::Error::new(
                    io::ErrorKind::Other,
                    "Stored account needs to contain 3 parameters!")
                );
        }
        let name: String = parts[0].to_string();
        let can_overdraw: bool = match parts[1].parse::<bool>() {
            Ok(val) => val,
            Err(_) => return Err(
                io::Error::new(io::ErrorKind::Other,
                "Second account value needs to be bool!")
            )
        };
        let description: String = parts[2].trim().to_string();
        let mut balance: f64 = 0.0;

        let mut transactions: Vec<Rc<Transaction>> = Vec::new();
        let mut recurring: Vec<Rc<Transaction>> = Vec::new();
        let mut transaction: Rc<Transaction>;
        let mut changed: bool = false;
        for res in reader.lines() {
            match res {
                Ok(line) => {
                    if line.len() < 1 { continue; }
                    transaction = match Transaction::load_from_string(line) {
                        Ok(mut t) => {
                            balance += t.amount;
                            t.update(&mut transactions);
                            Rc::new(t)
                        },
                        Err(e) => return Err(e)
                    };

                    match transaction.interval {
                        Some(_) => recurring.push(transaction.clone()),
                        None => {}
                    }
                },
                Err(e) => return Err(e)
            };
        }

        Ok(Account::new(name, description, path, balance, can_overdraw,
                        Box::<Vec<Rc<Transaction>>>::new(transactions)))
    }

    fn update_daily(t: &mut Transaction, ts: &mut Vec<Transaction>) {
        let mut transaction: Transaction;
        let mut last: DateTime<UTC>;

        match t.last_occurrence {
            Some(d) => {
                last = d + Duration::days(1);
                while last.date() < UTC::now().date() {
                    transaction = t.clone();
                    transaction.last_occurrence = Some(last);
                    ts.push(transaction);

                    last = last + Duration::days(1);
                }
            },
            None => return
        }
    }

    fn update_transaction(t: &mut Transaction, ts: &mut Vec<Transaction>) -> bool {
        Account::update_daily(t, ts);
        // match t.interval {
        //     Some(i) => {
        //         match i {
        //             Interval::Daily => {
        //                 Account::update_daily(&mut t, ts);
        //                 return true;
        //             },
        //             Interval::Weekly => {
        //                 if (t.date.date() + Duration::days(7)) < UTC::now().date() {
        //                     // t.date = UTC::now();
        //                     return true;
        //                 }
        //             },
        //             Interval::Biweekly => {
        //                 if (t.date.date() + Duration::days(14)) < UTC::now().date() {
        //                     // t.date = UTC::now();
        //                     return true;
        //                 }
        //             },
        //             Interval::Monthly => {
        //                 if t.date.month() < UTC::now().month() && t.date.day() <= UTC::now().day() {
        //                     // t.date = UTC::now();
        //                     return true;
        //                 }
        //             }
        //         }
        //     },
        //     None => {}
        // }

        return false;
    }

    pub fn save(&self) -> Result<&Account, io::Error> {
        let mut f = match OpenOptions::new().write(true).create(true).open(&self.filepath) {
            Ok(f) => f,
            Err(e) => { return Err(e); }
        };
        match write!(f, "{};{};{}\n", self.name, self.can_overdraw, self.description) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }

        for transaction in self.transactions.iter() {
            match write!(f, "{}", transaction.save_to_string()) {
                Ok(_) => {},
                Err(e) => return Err(e)
            };
        }

        Ok(self)
    }

    pub fn spent(&mut self, amount: f64, description: String)
        -> Result<&mut Account, AccountError> {
        let mut value = amount;
        if value > 0.0 { value *= -1.0; }
        if !self.can_overdraw && (self.balance + value) < 0.0 {
            return Err(AccountError::NoOverdraw( format!("Cannot overdraw {}", self.name) ));
        }
        self.transactions.push(Rc::new(Transaction::new(UTC::now(), value, description, None, None)));

        Ok(self)
    }

    pub fn got(&mut self, amount: f64, description: String)
        -> &mut Account {
        self.transactions.push(Rc::new(Transaction::new(UTC::now(), amount, description, None, None)));

        self
    }
}
