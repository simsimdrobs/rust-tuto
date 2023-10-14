fn main() {
    let mut user1 = User {
        name: String::from("Alice"),
        age: 28,
        account_balance: 750.0,
    };
    user1.greet();
    user1.deposit(250.0);
    if user1.is_eligible_for_discount() {
        println!("Congratulations! You are eligible for a discount.");
    } else {
        println!("Sorry, you are not eligible for a discount.");
    }

    toto();

    let message = String::from("Hello, borrowing!");
    // Borrowing with references
    let reference = &message;
    println!("Reference: {}", reference);
    // Mutable borrowing
    let mut mutable_reference = message;
    mutable_reference.push_str(" And mutation!");
    println!("Mutable Reference: {}", mutable_reference);
}

struct User {
    name: String,
    age: i32,
    account_balance: f64
}

impl User {
    fn greet(&self) {
        println!("Hello {} !", self.name);
    }

    fn deposit(&mut self, amount: f64) {
        self.account_balance += amount;
        println!("Deposited {:.2} units. New balance: {:.2}", amount, self.account_balance);
    }

    fn is_eligible_for_discount(&self) -> bool {
        self.age >= 60 || self.account_balance >= 10000.0
    }
}

fn toto() {
    let titi: String = "titi".to_string();
    let mut toto = titi;

    toto = "sas".to_string();
    println!("{}", toto);
}