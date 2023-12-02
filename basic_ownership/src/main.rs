fn greet(name: String) -> String {
    let mut result = String::from("Hello ");
    result.push_str(&name);
    return result;
}

fn greet_ref(name: &String) -> String {
    let mut result = String::from("Hello ");
    result.push_str(name);
    return result;
}

fn main() {
    let name = String::from("John Watson");
    let greeting = greet(name);
    // this will error!
    // println!("name: {}", name); // error: name is moved to greet function
    println!("result: {}", greeting);

    let name = String::from("Sherlock Holmes");
    let greeting = greet_ref(&name);
    println!("name: {}", name); // name is not moved
    println!("result: {}", greeting);
}
