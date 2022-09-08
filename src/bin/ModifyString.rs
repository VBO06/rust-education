fn main() {
    let s = String::from("Vincent");
    modify_string(&s);
}

fn modify_string(s: &String) {
    println!("{:?}, {:?}" , s, s);
    // String -> &str (as_str())
    let mut _x = s.as_str();
    let change_x = _x.replace("Vincent", "Toto");
    println!("{:?}" , change_x);
}