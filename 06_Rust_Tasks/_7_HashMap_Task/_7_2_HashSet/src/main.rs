use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

fn main() {
    let mut users = HashSet::new();

    // try_reserve
    match users.try_reserve(10) {
        Ok(_) => println!("Reserved capacity"),
        Err(e) => {
            eprintln!("Failed: {}", e);
            return;
        }
    }

    // insert
    users.insert(User { id: 1, name: "Alice".to_string(), active: true });
    users.insert(User { id: 2, name: "Bob".to_string(), active: false });

    // clone
    let cloned = users.clone();
    println!("Cloned: {:?}", cloned);

    // retain
    users.retain(|u| u.active);
    println!("After retain: {:?}", users);

    // extend
    users.extend([
        User { id: 3, name: "Carol".to_string(), active: true },
        User { id: 4, name: "Dave".to_string(), active: true },
    ]);
    println!("After extend: {:?}", users);

    // take (with derive, you need exact match)
    let lookup = User { id: 1, name: "Alice".to_string(), active: true };
    
    match users.take(&lookup) {
        Some(user) => println!("Taken: {:?}", user),
        None => println!("Not found"),
    }

    println!("Final: {:?}", users);
}