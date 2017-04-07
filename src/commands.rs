use arguments::Args;
use accountmanager::{print_account, print_overview};
use account::{Account, AccountError};

pub trait Command {
    fn execute(&self, account: Option<&mut Account>, args: &Args)
        -> Result<Option<Account>, AccountError>;
}

pub struct New {}
impl Command for New {
    fn execute(&self, account: Option<&mut Account>, args: &Args)
        -> Result<Option<Account>, AccountError> {
        let mut created: Account;

        match args.account {
            Some(ref a) => {
                created = match Account::create(a.clone(), args.description.clone(),
                                                             args.can_overdraw) {
                    Ok(a) => a,
                    Err(e) => { return Err(e); }
                };

                match created.save() {
                    Ok(a) => { println!("Created account {}", a.name); },
                    Err(e) => {
                        return Err(AccountError::FileError(e.to_string()));
                    }
                };
            },
            None => {
                return Err(AccountError::MissingOptions(
                    "You need to specify an account name".to_string()
                ));
            }
        }

        Ok(Some(created))
    }
}

pub struct Spent {}
impl Command for Spent {
    fn execute(&self, account: Option<&mut Account>, args: &Args)
        -> Result<Option<Account>, AccountError> {
        match args.amount {
            Some(a) => {
                match account {
                    Some(mut ac) => {
                        match ac.spent(a, args.description.clone()) {
                            Ok(_) => { },
                            Err(e) => { return Err(e); }
                        }
                    },
                    None => {
                        return Err(
                            AccountError::MissingOptions(
                                "You need to specify an account".to_string()
                            )
                        );
                    }
                }
            },
            None => {
                return Err(
                    AccountError::MissingOptions(
                        "You need to specify an amount".to_string()
                    )
                );
            }
        }

        Ok(None)
    }
}

pub struct Got {}
impl Command for Got {
    fn execute(&self, account: Option<&mut Account>, args: &Args)
        -> Result<Option<Account>, AccountError> {
        match args.amount {
            Some(a) => {
                match account {
                    Some(mut ac) => {
                        ac.got(a, args.description.clone());
                    },
                    None => { return Err(AccountError::MissingOptions(
                            "You need to specify an account".to_string()
                    )) }
                }
            },
            None => { return Err(AccountError::MissingOptions(
                "You need to specify an amount".to_string()
            )) }
        }
        Ok(None)
    }
}

// pub struct Show<'a> { pub name: String, pub accounts: &'a Vec<Account> }
// impl<'a> Command for Show<'a> {
//     fn execute(&self, account: Option<&mut Account>, args: &Args)
//         -> Result<Option<Account>, AccountError> {
//         match args.account {
//             Some(ref a) => {
//                 print_account(a.clone(), self.accounts);
//             },
//             None => { print_overview(self.accounts); }
//         }
//         Ok(None)
//     }
// }

pub struct Set {}
impl Command for Set {
    fn execute(&self, account: Option<&mut Account>, args: &Args)
        -> Result<Option<Account>, AccountError> {
        match account {
            Some(a) => {
                match args.amount {
                    Some(am) => {
                        a.set(am, args.description.clone());
                    },
                    None => { return Err(AccountError::MissingOptions(
                        "You need to specify an amount".to_string()
                    ))}
                }
            },
            None => { return Err(AccountError::MissingOptions(
                "You need to specify an account".to_string()
            ))}
        }

        Ok(None)
    }
}
