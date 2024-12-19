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

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InventoryError::FullInventory => write!(f, "Inventory is full"),
            InventoryError::ItemNotFound => write!(f, "Item not found"),
            InventoryError::InvalidSlot => write!(f, "Invalid equipment slot"),
            InventoryError::EquipError(e) => write!(f, "Equip error: {}", msg),
        }
    }
}
