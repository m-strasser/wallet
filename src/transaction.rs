use std::io;
use std::fmt;
use chrono::prelude::{DateTime, UTC};

#[derive (Debug)]
pub struct Transaction {
    pub date: DateTime<UTC>,
    pub amount: f64,
    pub description: String
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} @ {:.*}: {}", self.date.format("%Y-%m-%d"), 2, self.amount, self.description)
    }
}

impl Transaction {
    pub fn new(date: DateTime<UTC>, amount: f64, description: String) -> Transaction {
        Transaction {
            date: date,
            amount: amount,
            description: description
        }
    }

    pub fn load_from_string(line: String) -> Result<Transaction, io::Error> {
        let parts: Vec<&str> = line.split(';').collect();

        if(parts.len()) < 3 {
            return Err(
                io::Error::new(
                    io::ErrorKind::Other,
                    "Stored transaction needs to contain at least 3 parameters"
                )
            );
        }

        let date: DateTime<UTC> = match parts[0].parse::<DateTime<UTC>>() {
            Ok(val) => val,
            Err(_) => return Err(
                io::Error::new(io::ErrorKind::Other,
                "First transaction value needs to be convertable to chrono::DateTime<UTC>")
            )
        };

        let amount: f64 = match parts[1].parse::<f64>() {
            Ok(val) => val,
            Err(_) => return Err(
                io::Error::new(io::ErrorKind::Other,
                "Second transaction value needs to be convertable to float"
                )
            )
        };

        let description: String = parts[2].to_string();

        Ok(Transaction {
            date: date,
            amount: amount,
            description: description
        })
    }

    pub fn save_to_string(&self) -> String {
        format!("{:?}:{}:{}\n", self.date, self.amount, self.description).to_string()
    }
}
