struct BankAccount {
    account_holder: String,
    account_number: String,
    account_type: String,
    deposit_limit: f64,
    debit_limit: f64,
    balance: f64,
}

fn interest_rate(account_type: &str) -> f64 {
    match account_type {
        "Savings" => 6.04,
        "Current" => 7.03,
        "Fixed Deposit" => 8.15,
        _ => 0.0, // Default interest rate for unknown account types
    }
}

fn main() {
    let account = BankAccount {
        account_holder: String::from("Ashish Tiwari"),
        account_number: String::from("123456789"),
        account_type: String::from("Savings"),
        deposit_limit: 10000.0,
        debit_limit: 2000.0,
        balance: 5000.0,
    };

    println!("Account Holder: {}", account.account_holder);
    println!("Account Number: {}", account.account_number);
    println!("Account Type: {}", account.account_type);
    println!("Balance: {:.2}", account.balance);

    // implement  the interest  calculation  as  balance * rate * time / 100 as  interest  rate  depend on account type
    let rate = interest_rate(&account.account_type);
    let interest = account.balance * rate * 0.01;
    println!("Interest: {:.2}", interest);
}
