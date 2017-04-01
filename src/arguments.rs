/**
 * Handles the arguments passed to wallet.
 **/
use argparse::{ArgumentParser, Store, StoreTrue, Collect};
use std::fmt;

pub enum ArgsError {
    NoArgumentsSpecified( String )
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ArgsError::NoArgumentsSpecified(ref msg) => return write!(f, "{}", msg),
        };
    }
}

/**
 * Helper struct for arguments that relate to a transaction on an account.
 **/
pub struct AccountArgs {
    pub amount: Option<f64>,
    pub name: String
}

impl AccountArgs {
    pub fn from_string(s: String) -> Result<AccountArgs, ArgsError> {
        let parts: Vec<&str> = s.split(' ').collect();

        if parts.len() < 1 {
            return Err(ArgsError::NoArgumentsSpecified(String::from("No arguments given")));
        }

        let amount:Option<f64> = match parts[0].parse::<f64>() {
            Ok(v) => Some(v),
            Err(_) => None
        };

        match amount {
            Some(_) => return Ok(
                AccountArgs {
                    amount: amount,
                    name: parts[1..].join(" ")
                }),
            None => return Ok(AccountArgs { amount: amount, name: parts.join(" ") })
        };
    }
}

#[derive( Clone )]
pub struct Args {
    pub cmd: String,
    pub account: Option<String>,
    pub amount: Option<f64>,
    pub description: String,
    pub can_overdraw: bool
}

pub fn handle_args() -> Args {
    let mut cmd = String::new();
    let mut account = String::new();
    let mut can_overdraw = false;
    let mut description: String = String::from("No description");
    let mut str_args: Vec<String> = Vec::new();

    {
        let mut ap = ArgumentParser::new();

        ap.set_description("FINANCES");
        ap.refer(&mut cmd)
            .add_argument("COMMAND", Store, "Command to execute.")
            .required();
        ap.refer(&mut str_args)
            .add_argument("<args>", Collect, "The categories the transaction shall be placed in.");
        ap.refer(&mut account)
            .add_option(&["--account", "-a"], Store, "Account to operate on.");
        ap.refer(&mut description)
            .add_option(&["-d", "--description"], Store, "Description of the transaction or account.");
        ap.refer(&mut can_overdraw)
            .add_option(&["-o", "--overdraw"], StoreTrue, "Pass if the account can be overdrawn.");
        ap.parse_args_or_exit();
    }

    let mut arguments = Args {
        cmd: cmd.clone(), account: None, amount: None,
        description: "No description".to_string(),
        can_overdraw: can_overdraw.clone() };

    if account != "" { arguments.account = Some(account.clone()); }
    if description != "" { arguments.description = description.clone(); }
    if str_args.len() > 0 {
        arguments.amount = match str_args[0].parse::<f64>() {
            Ok(a) => Some(a),
            Err(_) => None
        };
    }

    return arguments;
}
