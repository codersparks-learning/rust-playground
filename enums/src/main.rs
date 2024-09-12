fn main() {
    println!("Hello, world!");

    //make_separator(&String::from("hello"));

    let o = Option::Some(String::from("Hello"));
    let x = get_or_default(&o);
    println!("String is {x}")
}

fn get_or_default(arg: &Option<String>) -> String {
    match arg {
        None => String::new(),
        Some(s) => s.clone(),
    }
}

// fn make_separator(user_str: &str) -> &str {
//     if user_str == "" {
//         let default = "=".repeat(10);
//         &default
//     } else {
//         user_str
//     }
// }
