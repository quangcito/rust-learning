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

// Item that can be stored in inventory
#[derive(Debug, Clone)]
pub struct Item {
    id: u32,
    name: String,
    item_type: ItemType,
    durability: u32,
    level_requirement: u32,
}

#[derive(Debug, Clone)]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
}

impl Item {
    pub fn new(id: u32, name: &str, item_type: ItemType, durability: u32, level_req: u32) -> Self {
        Item {
            id,
            name: name.to_string(),
            item_type,
            durability,
            level_requirement: level_req,
        }
    }

    pub fn use_item(&mut self) -> Result<(), String> {
        if self.durability == 0 {
            return Err("Item has no durability left".to_string());
        }
        self.durability -= 1;
        Ok(())
    }

    pub fn repair(&mut self, amount: u32) {
        self.durability += amount;
    }

    pub fn get_info(&self) -> String {
        format!(
            "{}(ID: {}) - Durability: {}/100, Level Req: {}",
            self.name, self.id, self.durability, self.level_requirement
        )
    }
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
