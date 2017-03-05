extern crate rand;

use std::rc::Rc;
use std::fs;

use account::Account;
use transaction::Transaction;
use interval::Interval;
use chrono::{UTC, Duration};

fn setUp() -> String {
    let p = rand::random::<u32>();
    format!("test_{}.finance", p).to_string()
}

fn tearDown(p: String) {
    fs::remove_file(p.clone());
}

#[test]
fn recurrence_daily_a_day_ago() {
    let p = setUp();
    println!("{}", p);
    let mut a = Account::new("Testaccount".to_string(),
                             "An account for testing.".to_string(),
                             p.clone(), 0.0, false,
                             Box::new(Vec::new()));
    let mut t = Rc::new(Transaction::new(UTC::now() - Duration::days(1), 1.0,
        "Test recurrence".to_string(), Some(Interval::Daily),
        Some(UTC::now() - Duration::days(1))
    ));

    a.transactions.push(t.clone());
    a.save();

    let mut b = Account::load(p.clone()).unwrap();
    tearDown(p);
    assert_eq!(b.transactions.len(), 1);
}

#[test]
fn recurrence_daily_a_week_ago() {
    let p = setUp();
    let mut a = Account::new("Testaccount".to_string(),
                             "An account for testing.".to_string(),
                             p.clone(), 0.0, false,
                             Box::new(Vec::new()));
    let mut t = Rc::new(Transaction::new(UTC::now() - Duration::days(7), 1.0,
        "Test recurrence".to_string(), Some(Interval::Daily),
        Some(UTC::now() - Duration::days(7))
    ));

    a.transactions.push(t.clone());
    a.save();

    let mut b = Account::load(p.clone()).unwrap();
    fs::remove_file(p.clone());
    tearDown(p);
    assert_eq!(b.transactions.len(), 6);
}

#[test]
fn recurrence_weekly_a_week_ago() {
    let p = setUp();
    let mut a = Account::new("Testaccount".to_string(),
                             "An account for testing.".to_string(),
                             p.clone(), 0.0, false,
                             Box::new(Vec::new()));
    let mut t = Rc::new(Transaction::new(UTC::now() - Duration::days(7), 1.0,
        "Test recurrence".to_string(), Some(Interval::Weekly),
        Some(UTC::now() - Duration::days(7))
    ));

    a.transactions.push(t.clone());
    a.save();

    let mut b = Account::load(p.clone()).unwrap();
    fs::remove_file(p.clone());
    tearDown(p);
    assert_eq!(b.transactions.len(), 1);
}
