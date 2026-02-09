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


pub struct Inventory<'a , T> {
    items: &'a mut HashMap<String, T>,
}

impl<'a , T> Inventory<'a , T>
where
    T: DisplayItem + Clone,
{
    pub fn new(items : &'a mut HashMap<String,T>) -> Self {
        Inventory {
            items
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
     //It Now accepts a closure as a parameter F where F: Fn(&HashMap<String, T>) -> String.
    pub fn display_all<F>(&self, display_closure: F) -> String 
    where
        F: Fn(&HashMap<String, T>) -> String,
    {
        display_closure(&self.items)
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

    let mut items : HashMap<String , Book> = HashMap::new();
    let mut inventory = Inventory::<Book>::new(&mut items);

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
    //Closure
    let display_closure = |items: &HashMap<String, Book>| -> String {
        if items.is_empty() {
            return "Inventory is empty.".to_string();
        }

        let header = "===== Inventory Items =====\n";
        
        let items_display = items
            .iter()
            .map(|(id, item)| {
                format!(
                    "ID: {}\n{}\n---------------------------\n",
                    id,
                    item.display()
                )
            })
            .collect::<Vec<String>>()
            .join("");

        format!("{}{}", header, items_display)
    };

    // Pass the closure to display_all
    println!("\n{}", inventory.display_all(display_closure));
}