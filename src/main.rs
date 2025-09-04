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
// enum BookType {
//     Fiction,
//     NonFiction,
//     Comics,
// }
// struct Book {
//     title: String,
//     author: String,
//     book_type: BookType,
//     is_available: bool,
// }
// impl Book {
//     fn print_info(&self) {
//         println!("Book :{}", self.title);
//         match self.book_type {
//             BookType::Comics => println!("This Book genre is Comics "),
//             BookType::Fiction => println!(" This Book genre is Fiction"),
//             BookType::NonFiction => println!(" This Book genre is Non Fiction"),
//         }
//         println!("Book author is :{}", self.author);
//         if self.is_available == true {
//             println!("This Book is available");
//         } else {
//             println!("This Book is not available");
//         }
//     }
//     fn borrow(&mut self) {
//         if self.is_available == false {
//             println!("This Book is already borrow");
//         }
//     }
//     fn return_book(&mut self) {
//         self.is_available = true;
//     }
// }
// fn main() {
//     let mut stanger_thing = Book {
//         title: String::from("StangerThing"),
//         author: String::from("Eleven"),
//         book_type: BookType::Fiction,
//         is_available: false,
//     };
//     stanger_thing.borrow();
//     stanger_thing.print_info();
// }
// fn main() {
//     let value = ("Hello", 5, "Good");
//     println!("{}", value.1);
//     let (name, age) = ("Talal", 21);
//     println!("{},{}", name, age);
// }
// fn coordinate() -> (i32, i32) {
//     (1, 7)
// }
// fn main() {
//     let (x, y) = coordinate();
//     if y > 5 {
//         println!(">5")
//     } else if y < 5 {
//         println!("<5");
//     } else if y == 5 {
//         println!("=5");
//     }
// }
// fn print(gt_100: bool) {
//     match gt_100 {
//         true => println!("The value is big"),
//         false => println!(" The value is small"),
//     }
// }
// fn main() {
//     let value = 100;
//     let is_gt_100 = value > 100;
//     print(is_gt_100);
// }
// #[derive(Debug)]
// enum Box {
//     red,
//     green,
//     blue,
// }
// struct Characteristics {
//     weight: i32,
//     color: Box,
// }
// impl Characteristics {
//     fn print(&self) {
//         println!("The Box is {},color:{:?} ", self.weight, self.color);
//     }
// }
// fn main() {
//     let value = Characteristics {
//         weight: 10,
//         color: Box::green,
//     };
//     value.print();
// }

use core::num;
use std::string;

// enum Color {
//     red,
//     green,
// }
// impl Color {
//     fn print(&self) {
//         match self {
//             Color::red => println!("This is red"),
//             Color::green => println!("This is green"),
//         }
//     }
// }
// struct Dimensions {
//     width: f64,
//     height: f64,
//     depth: f64,
// }
// impl Dimensions {
//     fn print(&self) {
//         println!("The width is {}", self.width);
//         println!("The height is {}", self.height);
//         println!("The depth is {}", self.depth);
//     }
// }
// struct ShippingBox {
//     color: Color,
//     dimensions: Dimensions,
//     weight: f64,
// }
// impl ShippingBox {
//     fn new(color: Color, dimensions: Dimensions, weight: f64) -> Self {
//         Self {
//             weight,
//             color,
//             dimensions,
//         }
//     }
//     fn print(&self) {
//         self.color.print();
//         self.dimensions.print();
//         println!("The weight is :{}", self.weight);
//     }
// }
// fn main() {
//     let Dimensions = Dimensions {
//         width: 5.0,
//         height: 10.5,
//         depth: 12.6,
//     };
//     let ShippingBox = ShippingBox::new(Color::green, Dimensions, 10.0);
//     ShippingBox.print();
// }
// enum MyType {
//     Int(i32),
//     Text(String),
// }
// fn print(numbers: Vec<MyType>) {
//     println!("The legth of the numbers is :{}", numbers.len());
// }
// fn main() {
//     let numbers = vec![
//         MyType::Int(10),
//         MyType::Int(20),
//         MyType::Text("Thrity".to_string()),
//         MyType::Int(40),
//     ];
//     for nums in &numbers {
//         match nums {
//             MyType::Int(val) => println!("Integer: {}", val),
//             MyType::Text(val) => println!("Text: {}", val),
//         }
//     }
//     print(numbers);
// }
// struct Person {
//     name: String,
//     age: i32,
//     color: String,
// }
// fn print(person: &Person) {
//     println!(
//         "The person name is {},color is ={}",
//         person.name, person.color
//     );
// }
// fn main() {
//     let people = vec![Person {
//         name: "Talal".to_owned(),
//         age: 21,
//         color: "Blue".to_owned(),
//     }];
//     let people = vec![Person {
//         name: "Ahmed".to_owned(),
//         age: 25,
//         color: "Green".to_owned(),
//     }];
//     let people = vec![Person {
//         name: "Ali".to_owned(),
//         age: 10,
//         color: "yellow".to_owned(),
//     }];
//     for name in &people {
//         if name.age <= 10 {
//             print(&name);
//         } else {
//             println!("Sorry your age is greater then 10");
//         }
//     }
// }
// struct Student {
//     name: Option<String>,
//     locker: Option<i32>,
// }
// fn print(student: &Student) {
//     match &student.name {
//         Some(n) => println!("The name of the student is: {}", n),
//         None => println!("Nothing is shown about the student "),
//     }
//     match &student.locker {
//         Some(n) => println!("The locker number of the student is: {}", n),
//         None => println!("No locker is not exist"),
//     }
// }
// fn main() {
//     let Locker = Student {
//         name: Some("talal".to_owned()),
//         locker: Some(24),
//     };
//     print(&Locker);
// }
#[derive(Debug)]
struct Adult {
    name: Option<String>,
    age: Option<i32>,
}
fn new(age: i32, name: &String) -> Result<Adult, String> {
    if age >= 21 {
        Ok(Adult {
            name: Some(name.to_owned()),
            age: Some(age),
        })
    } else {
        Err("Age must be at least 21".to_string())
    }
}
fn print(person: Result<Adult, String>) -> Result<String, String> {
    match person {
        Ok(adult) => match adult.age {
            Some(age) if age >= 21 => Ok("This person is Adult".to_string()),
            Some(age) => Err(format!(
                "The error is that this person :{} is not adult",
                age
            )),
            None => Err("Please enter a valid age".to_string()),
        },
        Err(e) => Err(e),
    }
}
fn main() {
    let result = new(21, &"Talal".to_owned());
    println!("{:?}", print(result));
}
