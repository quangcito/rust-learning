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

// An item in the game (like a sword or potion)
struct Item {
    id: u32,
    name: String,
    durability: u32,
}

// A slot where items can be equipped (like a weapon hand or armor slot)
struct EquipmentSlot {
    slot_type: String,
    // Notice the special wrappers - we'll explain these!
    equipped_item: Option<Rc<RefCell<Item>>>,
}

// The player's inventory (like a backpack)
struct Inventory {
    // This looks complex, but it makes sharing between threads safe
    items: Arc<Mutex<HashMap<u32, Item>>>,
    capacity: usize,
}
