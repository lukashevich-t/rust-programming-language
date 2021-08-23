fn main() {
    ownership();
}

fn ownership() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // compile error!


    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let l1 = (1, 2);
    let l2 = l1;
    println!("l1 = {:?}, l2 = {:?}", l1, l2);

    let l1 = (1, " ");
    let l2 = l1;
    println!("l1 = {:?}, l2 = {:?}", l1, l2);

    let l1 = (1, String::from("111"));
    let l2 = l1;
    // println!("l1 = {:?}, l2 = {:?}", l1, l2); // ошибка компиляции

    let s = String::from("hello");
    takes_ownership(s);
    // println!("s = {}", s); // ошибка компиляции
    let x = 5;
    makes_copy(x);
    println!("x = {}", x);



    let mut s1 = gives_ownership();
    println!("s1 = {}", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("s2 = {}", s2); // ошибка компиляции
    println!("s3 = {}", s3);

    let len = calculate_length(&s1);
    println!("s1 = {} (size={})", s1, len);

    try_change_borrowed(&mut s1);
    println!("changed s1 = {}", s1);
}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn try_change_borrowed(s: &mut String) {
    s.push_str("122");
}