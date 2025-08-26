// use std::io;
// fn sum(a: i32, b: i32) -> i32 {
//     a + b
// }
// fn sub(a: i32,b: i32)-> i32{
//     a-b
// }
// fn main() {
//     let mut input_a = String::new();
//     let mut input_b = String::new();
//     println!("What is your first number...");
//     io::stdin()
//         .read_line(&mut input_a)
//         .expect("Failed to read line");
//     println!("What is your second number...");
//     io::stdin()
//         .read_line(&mut input_b)
//         .expect("Failed to read line");
//     let a: i32 = input_a.trim().parse().expect("Please type a number!");
//     let b: i32 = input_b.trim().parse().expect("Please type a number!");
//     let result = sum(a, b);
//     println!("The sum is: {}", result);
//     let subtract=sub(a, b);
//     println!("The substract:{}",subtract);
// }
// fn main(){
// let value:bool=false;
// if value==true {
//     println!("hello");
// }
// else {
//     println!("goodbye");
// }
// }
// fn main(){
//     let value=5;

// if value>5 {
//     println!(">5");
// }
// else if value<5 {
//     println!("<5");
// }
// else if value==5 {
//     println!("=5");
// }
// else{
//     println!("Nothing found ");
// }
// }
// fn main(){
//     let my_name="Ahmed";
//     match my_name {
// "Alice"=>println!("not my name"),
// "Talal"=>println!("This is my name"),
// _=>println!("Nice to meet you"),
//     }
// }
// fn main(){
//     let value:bool=true;
//     match value{
//  true=>println!("This is true"),
//  false=>println!("This is false"),
//     }
// }
// fn main(){
//     let mut i=0;
// loop {
//     println!("The value is :{:?}",i);
//     i=i+1;
//     if i==4{
//         break;
//     }
// }
// }
// fn main() {
//     let mut i = 5;
//     while i >= 0 {
//         println!("The value is {:?}", i);
//         i = i - 1;
//         if i < 0 {
//             println!("Done");
//         }
//     }
// }
// enum Direction {
//     Left,
//     Right,
// }
// fn main() {
//     let go = Direction::Left;
//     match go {
//         Direction::Left => println!("Go left"),
//         Direction::Right => println!("Go right"),
//     }
// }
// enum Flavor {
//     sparkling,
//     sweet,
//     fruity,
// }
// struct Drink {
//     flavour: Flavor,
//     fluid_oz: f64,
// }
// fn print(drink: Drink) {
//     match drink.flavour {
//         Flavor::fruity => println!("This falvour is fruitty"),
//         Flavor::sparkling => println!("This flavour is sparkling"),
//         Flavor::sweet => println!("This falvour is sweet"),
//     }
//     println!("The fluid oz is :{}", drink.fluid_oz);
// }
// fn main() {
//     let drink = Drink {
//         flavour: Flavor::fruity,
//         fluid_oz: 40.02,
//     };
//     print(drink);
// }
// enum AccountType {
//     Savings,
//     Checkings,
// }
// struct BankAccount {
//     name: String,
//     account_type: AccountType,
//     balance: f64,
// }
// impl BankAccount {
//     fn deposit(&mut self, amount: f64) {
//         self.balance += amount;
//         println!("Deposited ${} to {}", amount, self.name);
//     }
//     fn withdraw(&mut self, amount: f64) {
//         if self.balance >= amount {
//             self.balance -= amount;
//             println!("Withdrew ${} from {}", amount, self.name);
//         } else {
//             println!("Insufficient funds for {}", self.name);
//         }
//     }
//     fn print_detail(&self) {
//         println!("Account: {}", self.name);
//         match self.account_type {
//             AccountType::Savings => println!("Type: Savings account"),
//             AccountType::Checkings => println!("Type: Checking account"),
//         }
//         println!("Balance: ${}\n", self.balance);
//     }
// }
// fn main() {
//     let mut alice = BankAccount {
//         name: String::from("Alice"),
//         account_type: AccountType::Savings,
//         balance: 1200.0,
//     };
//     let mut bob = BankAccount {
//         name: String::from("Bob"),
//         account_type: AccountType::Checkings,
//         balance: 500.0,
//     };
//     let mut charlie = BankAccount {
//         name: String::from("Charlie"),
//         account_type: AccountType::Checkings,
//         balance: 300.0,
//     };
//     alice.print_detail();
//     bob.print_detail();
//     charlie.print_detail();
//     bob.deposit(200.0);
//     charlie.withdraw(500.0);
//     alice.print_detail();
//     bob.print_detail();
//     charlie.print_detail();
// }
enum BookType {
    Fiction,
    NonFiction,
    Comics,
}
struct Book {
    title: String,
    author: String,
    book_type: BookType,
    is_available: bool,
}
impl Book {
    fn print_info(&self) {
        println!("Book :{}", self.title);
        match self.book_type {
            BookType::Comics => println!("This Book genre is Comics "),
            BookType::Fiction => println!(" This Book genre is Fiction"),
            BookType::NonFiction => println!(" This Book genre is Non Fiction"),
        }
        println!("Book author is :{}", self.author);
        if self.is_available == true {
            println!("This Book is available");
        } else {
            println!("This Book is not available");
        }
    }
    fn borrow(&mut self) {
        if self.is_available == false {
            println!("This Book is already borrow");
        }
    }
    fn return_book(&mut self) {
        self.is_available = true;
    }
}
fn main() {
    let mut stanger_thing = Book {
        title: String::from("StangerThing"),
        author: String::from("Eleven"),
        book_type: BookType::Fiction,
        is_available: false,
    };
    stanger_thing.borrow();
    stanger_thing.print_info();
}
// fn main() {
//     let value = ("Hello", 5, "Good");
//     println!("{}", value.1);
//     let (name, age) = ("Talal", 21);
//     println!("{},{}", name, age);
// }
