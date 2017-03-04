use transaction::Transaction;

use std::io;
use std::fmt;
use std::fs::{OpenOptions};
use std::io::{Write, BufReader, BufRead};
use chrono::prelude::UTC;

#[derive (Debug)]
pub enum AccountError {
    NoOverdraw( String )
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AccountError::NoOverdraw(ref msg) => return write!(f, "{}", msg),
        };
    }
}

#[derive (Debug)]
pub struct Account {
    pub name: String,
    description: String,
    filepath: String,
    balance: f64,
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
        let description: String = parts[2].to_string();
        let mut balance: f64 = 0.0;

        let mut transactions: Vec<Transaction> = Vec::new();
        for res in reader.lines() {
            match res {
                Ok(line) => {
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
        let mut f = match OpenOptions::new().write(true).open(&self.filepath) {
            Ok(f) => f,
            Err(e) => { return Err(e); }
        };
        match write!(f, "{};{};{}", self.name, self.can_overdraw, self.description) {
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
