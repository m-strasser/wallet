use interval::Interval;
use interval::interval_from_string;
use dateutils::last_day_of_month;

use std::rc::Rc;
use std::io;
use std::fmt;
use chrono::prelude::{DateTime, UTC};
use chrono::{Datelike, Timelike, Duration, TimeZone};

#[derive (Debug, Clone)]
pub struct Transaction {
    pub date: DateTime<UTC>,
    pub amount: f64,
    pub description: String,
    pub interval: Option<Interval>,
    pub last_occurrence: Option<DateTime<UTC>>
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*} @ {}: {}", 2, self.amount, self.date.format("%Y-%m-%d"), self.description)
    }
}

impl Transaction {
    pub fn update(&mut self, ts: &mut Vec<Rc<Transaction>>) -> &mut Transaction {
        let mut last: DateTime<UTC>;
        let mut t: Rc<Transaction>;

        match self.interval.clone() {
            Some(i) => {
                match self.last_occurrence {
                    Some(o) => {
                        match i {
                            Interval::Daily => {
                                self.push_occurrences(
                                    ts,
                                    Duration::days(1)
                                );
                            },
                            Interval::Weekly => {
                                self.push_occurrences(
                                    ts,
                                    Duration::days(7)
                                );
                            },
                            Interval::Biweekly => {
                                self.push_occurrences(ts, Duration::days(14));
                            },
                            Interval::Monthly => {
                                self.push_occurrences(ts, Transaction::until_next_month(o));
                            }
                        }
                    },
                    None => {}
                }
            },
            None => {}
        }

        return self;
    }

    fn until_next_month(d: DateTime<UTC>) -> Duration {
        let mut day = d.day();
        let mut month = d.month();
        let mut year = d.year();

        month = (month + 1) % 12;
        if month == 0 { month = 12; year += 1; }

        if day == last_day_of_month(month-1) {
            day = last_day_of_month(month);
        }

        Duration::days(day as i64)
    }

    fn push_occurrences(&mut self, ts: &mut Vec<Rc<Transaction>>, d: Duration) {
        let o = self.last_occurrence.unwrap();
        let mut last: DateTime<UTC> = o + d;
        let mut t: Rc<Transaction>;

        while last.date() < UTC::now().date() {
            t = Rc::new(Transaction::new(last, self.amount, self.description.clone(), None, None));
            ts.push(t);

            last = last + d;
        }

        self.last_occurrence = Some(last);
    }

    pub fn new(date: DateTime<UTC>, amount: f64, description: String, interval: Option<Interval>, last_occurrence: Option<DateTime<UTC>>) -> Transaction {
        Transaction {
            date: date,
            amount: amount,
            description: description,
            interval: interval,
            last_occurrence: last_occurrence
        }
    }

    pub fn load_from_string(line: String) -> Result<Transaction, io::Error> {
        let parts: Vec<&str> = line.split(';').collect();
        let mut interval: Option<Interval> = None;

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

        let description: String = parts[2].trim().to_string();

        if parts.len() > 3 {
            interval = match interval_from_string(parts[3].to_string()) {
                Ok(i) => Some(i),
                Err(e) => return Err(e)
            };
        }

        return Ok(Transaction {
            date: date,
            amount: amount,
            description: description,
            interval: interval,
            last_occurrence: Some(date) // @TODO: Store last occurrence
        });
    }

    pub fn save_to_string(&self) -> String {
        let mut string = format!("{:?};{};{}\n", self.date, self.amount, self.description).to_string();
        string = match self.interval {
            Some(ref i) => format!("{}:{:?}", string, i),
            None => format!("{}", string)
        };

        return string;
    }
}
