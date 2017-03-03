use std::io;
use std::fs::{OpenOptions};
use std::io::{Write, BufReader, BufRead};

#[derive (Debug)]
pub struct Account {
    name: String,
    filepath: String,
    balance: f64,
    can_overdraw: bool,
    transactions: Box<Vec<f64>>
}

impl Account {
    pub fn new(n: String, fp: String, b: f64, o: bool, ts: Box<Vec<f64>>) -> Account {
        Account {
            name: n,
            filepath: fp,
            balance: b,
            can_overdraw: o,
            transactions: ts
        }
    }

    pub fn load(path: String) -> Result<Account, io::Error> {
        let parts: Vec<&str> = path.split(':').collect();
        if (parts.len()) < 3 {
            return Err(io::Error::new(io::ErrorKind::Other,
                                  "Stored account needs to contain 3 parameters!"));
        }
        let name: String = parts[0].to_string();
        let path: String = parts[1].to_string();
        let can_overdraw: bool = match parts[2].parse::<bool>() {
            Ok(val) => val,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other,
                                                "Third account value needs to be bool!"))
        };

        let mut transactions: Vec<f64> = Vec::new();
        let f = match OpenOptions::new().read(true).open(&path){
            Ok(f) => f,
            Err(e) => return Err(e)
        };
        let reader = BufReader::new(&f);

        for res in reader.lines() {
            match res {
                Ok(line) => {
                    match line.parse::<f64>() {
                        Ok(amount) => transactions.push(amount),
                        Err(_) => return Err(io::Error::new(io::ErrorKind::Other,
                                                            "Transactions must be convertable to float!"))
                    };
                },
                Err(e) => return Err(e)
            };
        }

        Ok(Account::new(name, path, 0.0, can_overdraw, Box::<Vec<f64>>::new(transactions)))
    }

    pub fn save(&self) -> Result<&Account, io::Error> {
        let mut f = match OpenOptions::new().write(true).open(&self.filepath) {
            Ok(f) => f,
            Err(e) => { return Err(e); }
        };

        for transaction in self.transactions.iter() {
            match write!(f, "{:.*}\n", 2, transaction) {
                Ok(_) => {},
                Err(e) => return Err(e)
            };
        }

        Ok(self)
    }

    pub fn spent(&mut self, amount: f64) -> &mut Account {
        self.transactions.push(amount);

        self
    }
}
