#[derive(Debug)]
enum Sex {
    Male,
    Female,
}

#[derive(Debug)]
enum Info {
    Name(String),
    Age(u8),
    Location {x: i32, y: i32},
    Sex(Sex)
}


fn show_info(info: &Info) {
    match info {
        Info::Name(s) => println!("Name: {}", s),
        Info::Age(u) => println!("Age: {}", u),
        Info::Location { x, y } => println!("Location x: {}, y: {}", x, y),
        Info::Sex(sex) => println!("Sex: {:?}", sex),
    }
}


fn main() {
    show_info(&Info::Name(String::from("Jeon Kyubin")));
    show_info(&Info::Age(22));
    show_info(&Info::Location { x: 130, y: 132 });
    show_info(&Info::Sex(Sex::Male));
}