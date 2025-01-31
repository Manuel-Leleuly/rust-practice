mod first;
mod model;
mod second;
mod third;

fn main() {
    println!("Hello, world!");
    println!("Hello, Manuel!");
    println!("Hello, Eko!");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let name = "Manuel Theodore Leleuly";
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Manuel Theodore Leleuly";
    println!("Hello {}", name);

    name = "Budi Nugraha";
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Manuel Theodore Leleuly";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name)
}

/*
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
    ini komentar lebih dari satu baris
*/
#[test]
fn comment() {
    // ini komentar
    println!("Hello") // ini komentar lagi
}

#[test]
fn explicit() {
    let age: i32 = 20;
    println!("{}", age)
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("{}", a);

    let b: f32 = 10.234;
    println!("{}", b)
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = a as i32;
    println!("{}", c)
}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 10;
    let c = a * b;
    println!("{}", c);
    let d = a / b;
    println!("{}", d);
    let e = a + b;
    println!("{}", e)
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;
    println!("{} {}", a, b);
}

#[test]
fn comparison() {
    let a = 10;
    let b = 20;

    let result = a > b;
    println!("{}", result);
}

#[test]
fn boolean_operator() {
    let absen = 70;
    let nilai_akhir = 80;

    let lulus = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus_final = lulus && lulus_nilai_akhir;
    println!("{}", lulus_final);
}

#[test]
fn char_type() {
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2);
}

#[test]
fn tuple() {
    let mut data = (10, 10.5, true);
    println!("{:?}", data);

    // cara lama
    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    // destructure
    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;
    println!("{:?}", data);
}

fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);

    let test = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    let mut array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let length = array.len();
    println!("{}", length);
}

#[test]
fn two_dimensional_array() {
    let matrix = [[1, 2], [3, 4]];

    println!("{:?}", matrix);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[1][1]);
}

const MAXIMUM: i32 = 100;
#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MAXIMUM, MINIMUM)
}

#[test]
fn variable_scope() {
    let eko = 1;

    {
        // inner scope
        println!("inner eko: {}", eko);
        let kurniawan = 2;
        println!("inner kurniawan: {}", kurniawan);
    }

    // error karena var kurniawan berada di inner scope
    // println!("inner kurniawan: {}", kurniawan);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Leleuly");

    println!("{} {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Manuel");

    println!("{} {}", a, b);
}

#[test]
fn string() {
    let name = "   Manuel Theodore Leleuly   ";
    let trim = name.trim();

    println!("{}\n{}", name, trim);

    let mut username = "Manuel";
    println!("before: {}", username);
    username = "Leleuly";
    println!("after: {}", username);
}

#[test]
fn string_type() {
    let mut name = String::from("Manuel Theodore");
    println!("before: {}", name);

    name.push_str(" Leleuly");
    println!("after: {}", name);

    let new_name = name.replace("Manuel", "Toothbrush");
    println!("{} {}", name, new_name);
}

#[test]
fn ownership_rules() {
    // a tidak bisa diakses di sini, belum dideklarasikan
    let a = 10; // a bisa diakses mulai di sini

    {
        // b tidak bisa diakses di sini, belum dideklarasikan
        let b = 20; // b bisa diakses mulai di sini
        println!("{}", b);
    } // scope b selesai, b dihapus, b tidak bisa diakses lagi

    println!("{}", a);
} // scope a selesai, a dihapus, a tidak bisa diakses lagi

#[test]
fn data_copy() {
    let a = 10;
    let b = a; // copy data dari a ke b

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Manuel");

    // ownership dari name1 dipindahkan ke name2
    let name2 = name1;
    // name1 tidak bisa diakses di sini

    println!("{}", name2);
}

#[test]
fn clone() {
    let name1 = String::from("Manuel");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

#[test]
fn if_expression() {
    let value = 7;
    let result = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }
    }

    println!("{}", counter);
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };

    println!("Result: {}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;

    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter: {}", counter);
        }
        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array = ["A", "B", "C", "D", "E"];
    // let mut index = 0;

    // while index < array.len() {
    //     println!("Value: {}", array[index]);
    //     index += 1;
    // }

    for ele in array {
        println!("Value: {}", ele);
    }
}

#[test]
fn range() {
    let array = ["A", "B", "C", "D", "E"];

    let range = 0..array.len();
    println!("Start: {}, End: {}", range.start, range.end);

    for i in range {
        println!("{}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    let array = ["A", "B", "C", "D", "E"];

    let range = 0..=array.len();
    println!("Start: {}, End: {}", range.start(), range.end());

    for i in range {
        println!("{}", array[i]);
    }
}

fn say_hello() {
    println!("Hello World");
}

#[test]
fn test_say_hello() {
    say_hello();
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbyte {} {}", first_name, last_name);
}

#[test]
fn test_parameter() {
    say_goodbye("Manuel", "Leleuly");
    say_goodbye("Eko", "Khannedy");
    say_goodbye("Budi", "Santoso");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    return result;
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("{}", result);

    let result = factorial_loop(-5);
    println!("{}", result);
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("Manuel"), 10);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    return n * factorial_recursive(n - 1);
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("{}", result);
}

fn print_number(number: i32) {
    println!("number: {}", number);
}

fn hi(name: String) {
    println!("Hi, {}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Manuel");
    hi(String::clone(&name));
    println!("{}", name);
}

fn full_name(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Manuel");
    let last_name = String::from("Leleuly");

    let name = full_name(&first_name, &last_name);
    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn change_value(value: &mut String) {
    value.push_str("Test");
    // println!("{}", value);
}

#[test]
fn test_change_value() {
    let mut value = String::from("Manuel");

    let value_borrow = &mut value;

    change_value(value_borrow);
    change_value(value_borrow);
    change_value(value_borrow);

    println!("{}", value);
}

#[test]
fn slice_reference() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice1 = &array[..];
    println!("{:?}", slice1);

    let slice2 = &array[0..5];
    println!("{:?}", slice2);

    let slice3 = &array[5..];
    println!("{:?}", slice3);
}

#[test]
fn string_slice() {
    let name = String::from("Manuel Theodore Leleuly");

    let first_name = &name[0..3];
    println!("{}", first_name);

    let last_name = &name[14..];
    println!("{}", last_name);
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

#[test]
fn struct_person() {
    let first_name = String::from("Manuel");
    let last_name = String::from("Leleuly");

    let person = Person {
        first_name,
        middle_name: String::from("Theodore"),
        last_name,
        age: 20,
    };

    let person2 = Person {
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };

    print_person(&person);
    print_person(&person2);
}

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_associated_function() {
    let geo_point = GeoPoint::new(10.0, 10.0);
    println!("{} {}", geo_point.0, geo_point.1);
}

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-6.23223, 108.23232);
    println!("{} {}", geo_point.0, geo_point.1);
}

struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1 = Nothing;
    let _nothing2 = Nothing {};
}

#[test]
fn test_method() {
    let person = Person {
        first_name: String::from("Manuel"),
        middle_name: String::from("Theodore"),
        last_name: String::from("Leleuly"),
        age: 20,
    };

    person.say_hello("Eko");
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let level1 = Level::Regular;
    let _level2 = Level::Premium;
    let _level3 = Level::Platinum;

    match level1 {
        Level::Regular => {
            println!("Regular");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }
}

enum Payment {
    // card number
    CreditCard(String),
    // bank name, account number
    BankTransfer(String, String),
    // e-wallet name, e-wallet number
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: i32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!(
                    "Paying with bank transfer {} {} amount {}",
                    bank, number, amount
                );
            }
            Payment::EWallet(wallet, number) => {
                println!(
                    "Paying with e-wallet {} {} amount {}",
                    wallet, number, amount
                );
            }
        }
    }
}

#[test]
fn test_payment() {
    let payment1 = Payment::CreditCard(String::from("123123123"));
    let payment2 = Payment::BankTransfer(String::from("BCA"), String::from("123123123"));
    let payment3 = Payment::EWallet(String::from("Gopay"), String::from("31231231"));

    payment1.pay(1000);
    payment2.pay(1000);
    payment3.pay(1000);
}

#[test]
fn test_match_value() {
    let name = "Manuel";

    match name {
        "Manuel" => {
            println!("Hello Manuel");
        }
        other => {
            println!("Hello {}", other);
        }
    }

    match name {
        "Manuel" | "Eko" => {
            println!("Hello Boss");
        }
        other => {
            println!("Hello {}", other);
        }
    }
}

#[test]
fn test_range_patterns() {
    let value = 100;
    match value {
        75..=100 => {
            println!("Great");
        }
        50..=74 => {
            println!("Good");
        }
        25..=49 => {
            println!("Not Bad");
        }
        0..=24 => {
            println!("Bad");
        }
        other => {
            println!("Invalid value {}", other);
        }
    }
}

#[test]
fn test_struct_patterns() {
    let point = GeoPoint(0.0, 11.0);
    match point {
        GeoPoint(long, 0.0) => {
            println!("long: {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("lat: {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long: {} lat: {}", long, lat);
        }
    }

    let person = Person {
        first_name: String::from("Manuel"),
        middle_name: String::from("Theodore"),
        last_name: String::from("Leleuly"),
        age: 20,
    };

    match person {
        Person {
            first_name,
            last_name,
            ..
        } => {
            println!("First Name: {} Last Name: {}", first_name, last_name);
        }
    }
}

#[test]
fn test_ignoring() {
    let point = GeoPoint(0.0, 11.0);
    match point {
        GeoPoint(long, _) => {
            println!("long: {}", long);
        }
    }
}

#[test]
fn test_match_expression() {
    let value = 2;
    let result = match value {
        0 => "nol",
        1 => "satu",
        2 => "dua",
        _ => "invalid",
    };
    println!("{}", result);
}

type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

#[test]
fn test_customer() {
    let customer = Customer {
        id: String::from("123123123"),
        name: String::from("Manuel"),
        age: 20,
    };
    println!("{} {} {}", customer.id, customer.name, customer.age);
}

#[test]
fn test_module() {
    let user = model::User {
        first_name: String::from("Manuel"),
        last_name: String::from("Leleuly"),
        username: String::from("manuel.leleuly"),
        email: String::from("manuel@example.com"),
        age: 20,
    };
    user.say_hello("Eko");
}

use first::say_hello as say_hello_first;
use second::say_hello as say_hello_second;

#[test]
fn test_use() {
    // first::say_hello();
    // second::say_hello();
    say_hello_first();
    say_hello_second();
    first::second::third::say_hello();
}

trait CanSayHello {
    fn hello(&self) -> String {
        String::from("Hello")
    }
    fn say_hello(&self) -> String;
    fn say_hello_too(&self, name: &str) -> String;
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.first_name)
    }

    fn say_hello_too(&self, name: &str) -> String {
        format!("Hello, {}. My name is {}", name, self.first_name)
    }
}

fn say_hello_trait(value: &impl CanSayHello) {
    println!("{}", value.say_hello())
}

trait CanSayGoodbye {
    fn good_bye(&self) -> String;
    fn good_bye_too(&self, name: &str) -> String;
}

impl CanSayGoodbye for Person {
    fn good_bye(&self) -> String {
        format!("Good bye, my name is {}", self.first_name)
    }

    fn good_bye_too(&self, name: &str) -> String {
        format!("Good bye, {}. My name is {}", name, self.first_name)
    }
}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodbye)) {
    println!("{}", value.say_hello());
    println!("{}", value.good_bye());
}

#[test]
fn test_trait() {
    let person = Person {
        first_name: String::from("Manuel"),
        middle_name: String::from("Manuel"),
        last_name: String::from("Manuel"),
        age: 20,
    };

    say_hello_trait(&person);

    let result = person.say_hello_too("Eko");
    println!("{}", result);

    let result = person.hello();
    println!("{}", result);

    println!("{}", person.good_bye());
    println!("{}", person.good_bye_too("Eko"));

    hello_and_goodbye(&person);

    let person = Person {
        first_name: String::from("Manuel"),
        middle_name: String::from("Theodore"),
        last_name: String::from("Leleuly"),
        age: 20,
    };

    Person::say_hello(&person, "Eko");
    CanSayHello::say_hello(&person);
}

struct SimplePerson {
    name: String,
}

impl CanSayGoodbye for SimplePerson {
    fn good_bye(&self) -> String {
        format!("Good bye, my name is {}", self.name)
    }

    fn good_bye_too(&self, name: &str) -> String {
        format!("Good bye, {}. My name is {}", name, self.name)
    }
}

fn create_person(name: String) -> impl CanSayGoodbye {
    SimplePerson { name }
    // if name == String::from("Manuel") {
    //     SimplePerson { name }
    // } else {
    //     Person {
    //         first_name: name.clone(),
    //         last_name: name.clone(),
    //         middle_name: name.clone(),
    //         age: 20,
    //     }
    // }
}

#[test]
fn test_return_trait() {
    let person = create_person(String::from("Manuel"));
    println!("{}", person.good_bye());
    println!("{}", person.good_bye_too("Budi"));
}

trait CanSay: CanSayHello + CanSayGoodbye {
    fn say(&self) {
        println!("{}", self.say_hello());
        println!("{}", self.good_bye());
    }
}

struct Point<T = i32> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_struct() {
    let integer = Point::<i32> { x: 5, y: 10 };
    let float = Point::<f64> { x: 1.0, y: 4.0 };

    println!("integer x: {} y: {}", integer.x, integer.y);
    println!("float x: {} y: {}", float.x, float.y);
}

enum Value<T> {
    NONE,
    VALUE(T),
}

#[test]
fn test_generic_enum() {
    let value = Value::<i32>::VALUE(10);
    match value {
        Value::NONE => {
            println!("none");
        }
        Value::VALUE(value) => {
            println!("value: {}", value);
        }
    }
}

struct Hi<T: CanSayGoodbye> {
    value: T,
}

#[test]
fn test_generic_struct_with_trait() {
    let hi = Hi::<SimplePerson> {
        value: SimplePerson {
            name: String::from("Manuel"),
        },
    };
    println!("{}", hi.value.good_bye_too("Eko"));
}

fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn generic_in_function() {
    let result = min(10, 20);
    print!("{}", result);
}

#[test]
fn test_generic_method() {
    let point = Point { x: 10, y: 20 };
    println!("{}", point.get_x());
    println!("{}", point.get_y());
}

trait GetValue<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T {
        &self.x
    }
}

#[test]
fn test_generic_default_value() {
    let point = Point { x: 10, y: 20 };
    println!("x: {} y: {}", point.x, point.y);

    let point = Point::<f32> { x: 10.5, y: 20.5 };
    println!("x: {} y: {}", point.x, point.y);

    let point = Point { x: 10.5, y: 20.5 };
    println!("x: {} y: {}", point.x, point.y);
}

use core::ops::Add;
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    fmt::Error,
    ops::Deref,
    rc::Rc,
};

struct Apple {
    quantity: i32,
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity,
        }
    }
}

#[test]
fn test_operator_add() {
    let apple1 = Apple { quantity: 10 };
    let apple2 = Apple { quantity: 10 };

    let apple3 = apple1 + apple2;
    println!("{}", apple3.quantity);
}

fn double(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i * 2),
    }
}

#[test]
fn test_option() {
    let result = double(Some(10));
    println!("{:?}", result);

    let result = double(None);
    println!("{:?}", result);
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_compare() {
    let apple1 = Apple { quantity: 10 };
    let apple2 = Apple { quantity: 20 };

    println!("Apple1 == Apple2: {}", apple1 == apple2);
    println!("Apple1 < Apple2: {}", apple1 < apple2);
    println!("Apple1 > Apple2: {}", apple1 > apple2);
}

#[test]
fn test_string_manipulation() {
    let s = String::from("Manuel Theodore Leleuly");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.len());
    println!("{}", s.replace("Manuel", "Eko"));
    println!("{}", s.contains("Theodore"));
    println!("{}", s.starts_with("Manuel"));
    println!("{}", s.ends_with("Leleuly"));
    println!("{}", s.trim());
    println!("{:?}", s.get(0..3));
}
struct Category {
    id: String,
    name: String,
}

impl core::fmt::Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[test]
fn test_format() {
    let category = Category {
        name: String::from("Gadget"),
        id: String::from("GADGET"),
    };

    println!("{:?}", category);
}

#[test]
fn test_closure() {
    let sum = |value1: i32, value2: i32| -> i32 { value1 + value2 };

    let result = sum(1, 2);
    println!("Result: {}", result);
}

fn print_with_filter(value: String, filter: fn(String) -> String) {
    let result = filter(value);
    println!("Result: {}", result);
}

fn to_uppercase(value: String) -> String {
    value.to_uppercase()
}

#[test]
fn test_closure_as_parameter() {
    let name = String::from("Manuel Theodore Leleuly");
    let name2 = String::from("Manuel Theodore Leleuly");
    print_with_filter(name, |value| -> String { value.to_uppercase() });
    print_with_filter(name2, to_uppercase);
}

#[test]
fn test_closure_scope() {
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
        println!("Increment");
    };

    increment();
    increment();
    increment();

    println!("Counter: {}", counter);
}

struct Counter {
    counter: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.counter += 1;
        println!("Increment!");
    }
}

#[test]
fn test_closure_with_struct() {
    let mut counter = Counter { counter: 0 };
    counter.increment();
    counter.increment();
    counter.increment();

    println!("Counter: {}", counter.counter);
}

#[test]
fn test_vector() {
    let array = ["Manuel", "Theodore", "Leleuly"];

    for value in array {
        println!("{}", value);
    }

    println!("{:?}", array);

    let mut names = Vec::<String>::new();
    names.push(String::from("Manuel"));
    names.push(String::from("Theodore"));
    names.push(String::from("Leleuly"));

    for ele in &names {
        println!("{}", ele);
    }

    println!("{:?}", names);
}

#[test]
fn test_vec_deque() {
    let mut names = VecDeque::<String>::new();
    names.push_back(String::from("Manuel"));
    names.push_back(String::from("Theodore"));
    names.push_back(String::from("Leleuly"));

    for name in &names {
        println!("{}", name);
    }

    println!("{}", names[1]);
}

#[test]
fn test_linked_list() {
    let mut names = LinkedList::<String>::new();
    names.push_back(String::from("Manuel"));
    names.push_back(String::from("Theodore"));
    names.push_back(String::from("Leleuly"));

    for name in &names {
        println!("{}", name);
    }
}

#[test]
fn test_hash_map() {
    let mut map = HashMap::<String, String>::new();
    map.insert(String::from("name"), String::from("Manuel"));
    map.insert(String::from("age"), String::from("26"));
    map.insert(String::from("country"), String::from("Indonesia"));

    for entry in &map {
        println!("{} {}", entry.0, entry.1);
    }

    let name = map.get("name");
    let age = map.get("age");

    println!("Name: {}", name.unwrap());
    println!("Age: {}", age.unwrap());
}

#[test]
fn test_btree_map() {
    let mut map = BTreeMap::<String, String>::new();
    map.insert(String::from("name"), String::from("Manuel"));
    map.insert(String::from("age"), String::from("26"));
    map.insert(String::from("country"), String::from("Indonesia"));

    for entry in &map {
        println!("{} {}", entry.0, entry.1);
    }
}

#[test]
fn test_hash_set() {
    let mut set = HashSet::<String>::new();
    set.insert(String::from("Manuel"));
    set.insert(String::from("Manuel"));
    set.insert(String::from("Theodore"));
    set.insert(String::from("Theodore"));
    set.insert(String::from("Leleuly"));
    set.insert(String::from("Leleuly"));

    for value in &set {
        println!("{}", value);
    }
}

#[test]
fn test_btree_set() {
    let mut set = BTreeSet::<String>::new();
    set.insert(String::from("Manuel"));
    set.insert(String::from("Manuel"));
    set.insert(String::from("Theodore"));
    set.insert(String::from("Theodore"));
    set.insert(String::from("Leleuly"));
    set.insert(String::from("Leleuly"));

    for value in &set {
        println!("{}", value);
    }
}

#[test]
fn test_iterator() {
    let array = [1, 2, 3, 4, 5];
    let mut iterator = array.iter();

    while let Some(value) = iterator.next() {
        println!("iterator: {}", value);
    }

    for value in array {
        println!("for: {}", value);
    }
}

#[test]
fn test_iterator_method() {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Vector: {:?}", vector);

    let sum: i32 = vector.iter().sum();
    println!("Sum: {}", sum);

    let count = vector.iter().count();
    println!("Count: {}", count);

    let doubled: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", odd);
}

fn connect_database(host: Option<String>) {
    match host {
        Some(host) => {
            println!("Connecting to database {}", host);
        }
        None => {
            panic!("No database host provided");
        }
    }
}

#[test]
fn test_unrecoverable_error() {
    connect_database(Some(String::from("mysql")));
}

fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => Ok(host),
        None => Err(String::from("No cache host provided")),
    }
}

fn connect_email(host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => Ok(host),
        None => Err(String::from("No email host provided")),
    }
}

fn connect_application(host: Option<String>) -> Result<String, String> {
    // let connect_cache = connect_cache(host.clone());
    // match connect_cache {
    //     Ok(_) => {}
    //     Err(err) => {
    //         return Err(err);
    //     }
    // }

    // let connect_email = connect_email(host.clone());
    // match connect_email {
    //     Ok(_) => {}
    //     Err(err) => {
    //         return Err(err);
    //     }
    // }

    connect_cache(host.clone())?;
    connect_email(host.clone())?;
    Ok("Connected to application".to_string())
}

#[test]
fn test_recoverable_error() {
    let cache = connect_cache(None);
    match cache {
        Ok(host) => {
            println!("Connected to cache at {}", host);
        }
        Err(err) => {
            println!("Error connecting to cache: {}", err);
        }
    }
}

#[test]
fn test_connect_app() {
    // let result = connect_application(Some("localhost".to_string()));
    let result = connect_application(None);
    match result {
        Ok(msg) => {
            println!("{}", msg);
        }
        Err(err) => {
            println!("Error connecting to application: {}", err);
        }
    }
}

// #[test]
// fn test_dangling_reference() {
//     let r: &i32;
//     {
//         let x = 5;
//         // r = &x; // pasti error
//     }
//     println!("{}", r);
// }

fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str {
    if value1.len() > value2.len() {
        value1
    } else {
        value2
    }
}

#[test]
fn test_lifetime_annotation() {
    let value1 = "Manuel";
    let value2 = "Leleuly";

    let result = longest(value1, value2);
    println!("{}", result);
}

#[test]
fn test_lifetime_annotation_dangling_reference() {
    let string1 = String::from("Manuel");
    let string2 = String::from("Leleuly");
    let result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("{}", result);
}

struct Student<'a, 'b> {
    name: &'a str,
    last_name: &'b str,
}

impl<'a, 'b> Student<'a, 'b> {
    fn longest_name(&self, student: &Student<'a, 'b>) -> &'a str {
        if self.name.len() > student.name.len() {
            self.name
        } else {
            student.name
        }
    }
}

fn longest_student_name<'a, 'b>(student1: &Student<'a, 'b>, student2: &Student<'a, 'b>) -> &'a str {
    if student1.name.len() > student2.name.len() {
        student1.name
    } else {
        student2.name
    }
}

#[test]
fn test_student() {
    let student = Student {
        name: "Manuel",
        last_name: "Leleuly",
    };
    println!("{}", student.name);
    let student2 = Student {
        name: "Eko",
        last_name: "Khannedy",
    };
    let result = longest_student_name(&student, &student2);
    println!("{}", result);

    let result = student.longest_name(&student2);
    println!("{}", result);
}

struct Teacher<'a, Id>
where
    Id: Ord,
{
    id: Id,
    name: &'a str,
}

#[test]
fn test_lifetime_annotation_generic() {
    let teacher = Teacher {
        id: 10,
        name: "Manuel",
    };

    println!("{} {}", teacher.id, teacher.name);
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
    name: String,
    location: String,
    website: String,
}

#[test]
fn test_attribute_debut() {
    let company = Company {
        name: "Programmer Zaman Now".to_string(),
        location: "Indonesia".to_string(),
        website: "https://www.programmerzamannow.com".to_string(),
    };
    println!("{:?}", company);

    let company2 = Company {
        name: "Programmer Zaman Now".to_string(),
        location: "Indonesia".to_string(),
        website: "https://www.programmerzamannow.com".to_string(),
    };
    println!("{:?}", company2);

    let result = company == company2;
    println!("{}", result);
}

#[test]
fn test_box() {
    let value = Box::new(10);
    println!("value {}", value);
    display_number(*value);
    display_number_reference(&value);
}

fn display_number(value: i32) {
    println!("{}", value);
}

fn display_number_reference(value: &i32) {
    println!("{}", value);
}

#[derive(Debug)]
enum ProductCategory {
    Of(String, Box<ProductCategory>),
    End,
}

#[test]
fn test_box_enum() {
    let category = ProductCategory::Of(
        "Laptop".to_string(),
        Box::new(ProductCategory::Of(
            "Dell".to_string(),
            Box::new(ProductCategory::End),
        )),
    );
    println!("{:?}", category);
    print_category(&category);
}

fn print_category(category: &ProductCategory) {
    println!("{:?}", category)
}

#[test]
fn test_dereference() {
    let value1 = Box::new(10);
    let value2 = Box::new(20);

    let result = *value1 * *value2;
    println!("{}", result);
}

struct MyValue<T> {
    value: T,
}

impl<T> Deref for MyValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[test]
fn test_deref() {
    let value = MyValue { value: 20 };
    let real_value = *value;
    println!("value: {}", real_value);
}

fn say_hello_reference(name: &String) {
    println!("Hello {}", name);
}

#[test]
fn test_deref_reference() {
    let name = MyValue {
        value: String::from("Manuel"),
    };
    say_hello_reference(&name);
}

struct Book {
    title: String,
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Dropping book: {}", self.title);
    }
}

#[test]
fn test_drop() {
    let book = Book {
        title: String::from("Rust Programming"),
    };
    println!("{}", book.title);
}

enum Brand {
    Of(String, Rc<Brand>),
    End,
}

#[test]
fn test_multiple_ownership() {
    let apple = Rc::new(Brand::Of(String::from("Apple"), Rc::new(Brand::End)));
    println!("Apple reference count: {}", Rc::strong_count(&apple));

    let laptop = Brand::Of(String::from("Laptop"), Rc::clone(&apple));
    println!("Apple reference count: {}", Rc::strong_count(&apple));

    {
        let smartphone = Brand::Of(String::from("Smartphone"), Rc::clone(&apple));
        println!("Apple reference count: {}", Rc::strong_count(&apple));
    }

    println!("Apple reference count: {}", Rc::strong_count(&apple));

    // let apple = ProductCategory::Of(String::from("Apple"), Box::new(ProductCategory::End));
    // let laptop = ProductCategory::Of(String::from("Laptop"), Box::new(apple));
    // let smartphone = ProductCategory::Of(String::from("Smartphone"), Box::new(apple));
}

#[derive(Debug)]
struct Seller {
    name: RefCell<String>,
    active: RefCell<bool>,
}

#[test]
fn test_ref_cell() {
    let seller = Seller {
        name: RefCell::new(String::from("Manuel")),
        active: RefCell::new(true),
    };

    let mut result = seller.name.borrow_mut();
    *result = String::from("Eko");

    println!("{:?}", seller);
}

static APPLICATION: &str = "My Application";

#[test]
fn test_static() {
    println!("Application: {}", APPLICATION);
}

static mut COUNTER: u32 = 0;

unsafe fn increment() {
    COUNTER += 1;
}

#[test]
fn test_unsafe() {
    unsafe {
        increment();
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    };
}

macro_rules! hello {
    () => {
        println!("Hi");
    };
    ($name: expr) => {
        println!("Hi {}!", $name);
    };
}

#[test]
fn test_macro() {
    hello!();
    hello!("Manuel");
    hello! {
        "Manuel"
    };

    let name = "Manuel";
    hello!(name);
}

macro_rules! iterate {
    ($array:expr) => {
        for i in $array {
            println!("{}", i);
        }
    };
    ($($item:expr), *) => {
        $(
            println!("{}", $item);
        )*
    }
}

#[test]
fn test_iterate() {
    iterate!([1, 2, 3, 4, 5]);
    iterate!(10, 9, 8, 7, 6);
}
