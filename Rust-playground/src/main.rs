use classTest::Post;

fn main() {
    let s = String::from("Hello");
    string_test(&s);
    test(5, 10);

    let test = classTest::Car{};
}

fn test(number1: i32, number2: i32){
    println!("{}", number1 + number2);
}

fn string_test(text: &str){
    println!("{}", text);
}