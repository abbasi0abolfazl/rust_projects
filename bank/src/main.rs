
#[derive(Debug)]
struct Account{
    id: u32,
    balance: i32,
    holder: String
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account{
            id,
            holder,
            balance: 0
        }
    }
    
}

#[derive(Debug)]
struct Bank{
    accounts: Vec<Account>
}

impl Bank {
    fn new() -> Self{
        Bank{accounts: vec![]}
    }
}

/// Helper function for pretty print 
fn print_account(account:Account){
    println!("{:#?}", account);
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(
        1,
        String::from("abolfazl")
    );

    print_account(account);
}
