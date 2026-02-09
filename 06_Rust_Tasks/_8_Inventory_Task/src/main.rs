use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum InventoryError {
    Duplicated(String),
    InvalidId(String),
    ItemNotFound(String),
}


impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::Duplicated(id) => {
                write!(f, "An item with ID '{}' already exists.", id)
            }
            InventoryError::InvalidId(id) => {
                write!(f, "The provided ID '{}' is invalid.", id)
            }
            InventoryError::ItemNotFound(id) => {
                write!(f, "No item found with ID '{}'.", id)
            }
        }
    }
}


pub trait DisplayItem {
    fn display(&self) -> String;
}


pub struct Inventory<T> {
    items: HashMap<String, T>,
}

impl<T> Inventory<T>
where
    T: DisplayItem + Clone,
{
    pub fn new() -> Self {
        Inventory {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, id: String, item: T) -> Result<(), InventoryError> {
        if id.trim().is_empty() {
            return Err(InventoryError::InvalidId(id));
        }

        if self.items.contains_key(&id) {
            return Err(InventoryError::Duplicated(id));
        }

        self.items.insert(id, item);
        Ok(())
    }

    pub fn display_all(&self) -> String {
        if self.items.is_empty() {
            return "Inventory is empty.".to_string();
        }

        let mut output = String::from("===== Inventory Items =====\n");

        for (id, item) in &self.items {
            output.push_str(&format!(
                "ID: {}\n{}\n---------------------------\n",
                id,
                item.display()
            ));
        }

        output
    }
}



#[derive(Clone)]
struct Book {
    title: String,
    author: String,
}

impl DisplayItem for Book {
    fn display(&self) -> String {
        format!("Title : {}\nAuthor: {}", self.title, self.author)
    }
}



fn main() {
    let mut inventory = Inventory::<Book>::new();

    let book1 = Book {
        title: "Rust Book".to_string(),
        author: "Steve".to_string(),
    };

    let book2 = Book {
        title: "Rust Book".to_string(),
        author: "Steve".to_string(),
    };

    match inventory.add_item("1".to_string(), book1) {
        Ok(_) => println!("✔ Item added successfully."),
        Err(e) => println!("✖ Error: {}", e),
    }

    match inventory.add_item("1".to_string(), book2) {
        Ok(_) => println!("✔ Item added successfully."),
        Err(e) => println!("✖ Error: {}", e),
    }

    println!("\n{}", inventory.display_all());
}
