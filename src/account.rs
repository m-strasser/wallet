use transaction::Transaction;

use std::io;
use std::fmt;
use std::fs::{OpenOptions};
use std::io::{Write, BufReader, BufRead};
use chrono::prelude::UTC;

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
    transactions: Box<Vec<Transaction>>
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({}): {}", self.name, self.balance, self.description)
    }
}

impl Account {
    pub fn new(n: String, d: String, fp:String, b: f64, o: bool, ts: Box<Vec<Transaction>>)
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
            transactions: Box::<Vec<Transaction>>::new(Vec::new())
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

        let mut transactions: Vec<Transaction> = Vec::new();
        for res in reader.lines() {
            match res {
                Ok(line) => {
                    if line.len() < 1 { continue; }
                    match Transaction::load_from_string(line) {
                        Ok(t) => { balance += t.amount; transactions.push(t); },
                        Err(e) => return Err(e)
                    };
                },
                Err(e) => return Err(e)
            };
        }

        Ok(Account::new(name, description, path, balance, can_overdraw,
                        Box::<Vec<Transaction>>::new(transactions)))
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
        self.transactions.push(Transaction::new(UTC::now(), value, description));

        Ok(self)
    }

    pub fn got(&mut self, amount: f64, description: String)
        -> &mut Account {
        self.transactions.push(Transaction::new(UTC::now(), amount, description));

        self
    }
}
