use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self { 
            inner: HashMap::new() 
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.clone(), bill);
    }
    
    fn get_all(&self) -> Vec<&Bill> {
        let mut bills = vec![];
        for bill in self.inner.values() {
            bills.push(bill);
        }
        bills
    } 

    fn update(&mut self, name: &str, amount:f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }

    fn remove(&mut self, name: &str) -> bool {
        // is_some() return if object was perfectly removed
        self.inner.remove(name).is_some()
    }

}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter e number"),
        }
    }
}

/**
    & borrow (because we need to use bills after this method)
    mut : mutable because we are going to change the structure of bills on this method 
**/
fn add_bill_menu(bills: &mut Bills) {
    // get the bill name 
    println!("Enter the bill name you want to add :");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    // get the bill amount 
    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };
    let bill = Bill { name, amount};
    bills.add(bill);
    println!("Bill added"); 
}

fn view_bills_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }

}

fn update_bills_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter name of bill to update:");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };
    println!("Enter the amount of bill to update:");
    let amount = match get_input() {
        Some(amount) => amount,
        None => return,
    };

    
}

fn remove_bills_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter bill name to remove:");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };
    if bills.remove(&name) {
        println!("removed");
    } else {
        println!("bill not found");
    }
}


fn main_menu() {
    fn show() {
        println!("");
        println!("-- Manage Bill --");
        println!("1. Add Bill");
        println!("2. View Bill");
        println!("3. Update Bill");
        println!("");
        println!("Enter selection:");
    }
    
    let mut bills = Bills::new();

    loop {
        show();
      MyBillImplementation  let input = match get_input() {
            Some(input) => input,
            None => String::new(),
        };
        // String -> &str
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            "3" => remove_bills_menu(&mut bills),
            "4" => update_bills_menu(&mut bills),
            _ => println!(""),
        }
    }
}


fn main() {
   main_menu();
}