struct Person<'a> {
    name: &'a str,
    password: &'a str,
}

fn main() {
    let name = String::from("Alice");
    let password = String::from("secret");
    let person = Person {
        name: &name,
        password: &password,
    };
    println!("Person: name: {}, password: {} ", person.name, person.password);
    let s1 = String::from("Hello, ");
    let ans;
    {
        let s2 = String::from("Woa");
        ans = long_length(&s1, &s2);
        println!("Answer: {}", ans);
    }
    //  print!("Answer: {}", ans);
}

fn long_length<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        return s1;
    } else {
        return s2;
    }
}
