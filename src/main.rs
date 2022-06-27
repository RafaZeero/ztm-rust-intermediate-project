struct BankAccount {
    balance: i32,
    verified: bool,
}

fn print_balance(account: &BankAccount) {
    println!("{:?}", account.balance);
}

fn print_verified(account: &BankAccount) {
    println!("{:?}", account.verified);
}

fn is_verified(account: &BankAccount) -> Result<bool, bool> {
    return match account.verified {
        true => Ok(true),
        false => Err(false),
    };
}

fn main() {
    let my_account = BankAccount {
        balance: 50000,
        verified: true,
    };

    // this variable will test the panic codition on Rust
    // let my_account2 = BankAccount {
    //     balance: 50000,
    //     verified: false,
    // };

    let verification_status = is_verified(&my_account).unwrap();

    // this will make Rust panic and throw an error
    // let verification_status2 = is_verified(&my_account2).expect("<Write and error message here>");

    // println!("{:?}", my_account.balance);
    // println!("{:?}", my_account.verified);

    print_balance(&my_account);
    print_verified(&my_account);
    println!("{:?}", verification_status);

    // the next line will print the error
    // println!("{:?}", verification_status2);
}
