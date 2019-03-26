use commonlib;

fn main() {
    let mut structtest = commonlib::structtest::StructTest::new();
    structtest.name = "Joe".to_string();
    println!("{}", structtest.say_hello());
}
