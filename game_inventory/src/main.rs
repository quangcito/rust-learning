fn main() {
    println!("Hello, world!");
}

// Custome error type for inventory operations
#[derive(Debug)]
pub enum InventoryError {
    FullInventory,
    ItemNotFound,
    InvalidSlot,
    EquipError(String),
}
