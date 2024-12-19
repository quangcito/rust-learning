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

impl EquipmentSlot {
    pub fn new(slot_type: &str) -> Self {
        EquipmentSlot {
            slot_type: slot_type.to_string(),
            equipped_item: None,
        }
    }

    pub fn equip(&mut self, item: Rc<RefCell<Item>>) -> Result<Option<Rc<RefCell<Item>>>, String> {
        // Check if item meets requirements
        let borrowed_item = item.borrow();
        match &borrowed_item.item_type {
            ItemType::Weapon if self.slot_type != "Weapon" => {
                return Err("Cannot equip weapon in this slot".to_string())
            }
            ItemType::Armor if self.slot_type != "Armor" => {
                return Err("Cannot equip armor in this slot".to_string())
            }
            _ => {}
        }

        Ok(self.equipped_item.replace(item))
    }

    pub fn unequip(&mut self) -> Option<Rc<RefCell<Item>>> {
        self.equipped_item.take()
    }

    pub fn get_equipped_info(&self) -> String {
        match &self.equipped_item {
            Some(item) => format!("{}: {}", self.slot_type, item.borrow().get_info()),
            None => format!("{}: Empty", self.slot_type),
        }
    }
}

// The player's inventory (like a backpack)
struct Inventory {
    // This looks complex, but it makes sharing between threads safe
    items: Arc<Mutex<HashMap<u32, Item>>>,
    capacity: usize,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Inventory {
            items: Arc::new(Mutex::new(HashMap::new())),
            capacity,
        }
    }

    pub fn add_item(&self, item: Item) -> Result<(), InventoryError> {
        let mut items = self.items.lock().unwrap();
        if items.len() >= self.capacity {
            return Err(InventoryError::FullInventory);
        }
        items.insert(item.id, item);
        Ok(())
    }

    pub fn remove_item(&self, item_id: u32) -> Result<Item, InventoryError> {
        let mut items = self.items.lock().unwrap();
        items.remove(&item_id).ok_or(InventoryError::ItemNotFound)
    }

    pub fn get_item(&self, item_id: u32) -> Option<Item> {
        let items = self.items.lock().unwrap();
        items.get(&item_id).cloned()
    }

    pub fn list_items(&self) -> Vec<Item> {
        let items = self.items.lock().unwrap();
        items.values().cloned().collect()
    }
}

// Player stats
#[derive(Debug)]
pub struct PlayerStats {
    level: u32,
    strength: u32,
    defense: u32,
}

// Player that owns both inventory and equipment
pub struct Player {
    name: String,
    inventory: Inventory,
    equipment: HashMap<String, EquipmentSlot>,
    stats: RefCell<PlayerStats>,
}
