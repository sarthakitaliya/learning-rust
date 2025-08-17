use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn main() {
    let user = User {
        id: 1,
        name: String::from("Alice"),
        email: String::from("example@gmail.com"),
    };
    let mut buffer: Vec<u8> = Vec::new();
    let ans = user.serialize(&mut buffer);

    match ans {
        Ok(_) => println!("Serialized user: {:?}", buffer),
        Err(_) => eprintln!("Error serializing user"),
    }
    let deserialized: User = User::try_from_slice(&buffer).unwrap();

    assert_eq!(user, deserialized);
    println!(
        "Successfully serialized and deserialized: {:?}",
        deserialized
    );
}
