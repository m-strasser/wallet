# Wallet

A simple command line utility to manage your personal finances.

Implemented functions:

`new` - Creates a new account.
`spent` - Subtracts the given amount from an account.
`got` - Adds the given amount to an account.
`set` - Sets an account to a given amount.
`show` - Shows the transactions that happened on an account.

Example commands:

`wallet spent 50 -a my_account --description "Bought some stuff"`
`wallet got 50 -a my_other_account -d "Earned money"`
