use std::collections::HashMap;

#[derive(Debug, Clone)]
struct User {
    name: String,
    active: bool,
}

fn main() {
    // User ID is the stable key
    let mut users: HashMap<u32, User> = HashMap::new();

    // try_reserve: safely reserve capacity
    if let Err(e) = users.try_reserve(10) {
        println!("Failed to reserve capacity: {}", e);
        return;
    }

    // Insert users
    users.insert(
        1,
        User {
            name: "Alice".to_string(),
            active: true,
        },
    );

    users.insert(
        2,
        User {
            name: "Bob".to_string(),
            active: false,
        },
    );

    // clone: clone the entire HashMap
    let cloned_users = users.clone();
    println!("Cloned users: {:?}", cloned_users);

    // retain: keep only active users
    users.retain(|_, user| user.active);
    println!("After retain (only active): {:?}", users);

    // extend: add users from another source
    let mut new_users: HashMap<u32, User> = HashMap::new();
    new_users.insert(
        3,
        User {
            name: "Carol".to_string(),
            active: true,
        },
    );

    users.extend(new_users);
    println!("After extend: {:?}", users);

    // take: safely take ownership of a user value
    // let taken_user = users.get_mut(&1).map(|user| std::mem::take(user));

    let removed_u = users.remove(&2);
    if let Some(user) = removed_u {
        println!("Taken user: {:?}", user);
    } else {
        println!("User ID not found");
    }

    println!("Final users map: {:?}", users);
}
