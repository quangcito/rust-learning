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
    // This makes sharing between threads safe
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

impl Player {
    pub fn new(name: &str, inventory_capacity: usize) -> Self {
        let mut equipment = HashMap::new();
        equipment.insert("Weapon".to_string(), EquipmentSlot::new("Weapon"));
        equipment.insert("Armor".to_string(), EquipmentSlot::new("Armor"));

        Player {
            name: name.to_string(),
            inventory: Inventory::new(inventory_capacity),
            equipment,
            stats: RefCell::new(PlayerStats {
                level: 1,
                strength: 10,
                defense: 10,
            }),
        }
    }

    pub fn equip_item(&mut self, item_id: u32, slot: &str) -> Result<(), InventoryError> {
        // Check if slot exists
        let equipment_slot = self.equipment.get_mut(slot)
            .ok_or(InventoryError::InvalidSlot)?;

        // Remove item from inventory
        let item = self.inventory.remove_item(item_id)?;

        // Create shared reference
        let item_rc = Rc::new(RefCell::new(item));

        // Try to equip item
        match equipment_slot.equip(item_rc) {
            Ok(previous_item) => {
                // If there was a previous item, add it back to inventory
                if let Some(prev_item) = previous_item {
                    let item = Rc::try_unwrap(prev_item)
                        .map_err(|_| InventoryError::EquipError("Failed to unequip item".to_string()))?
                        .into_inner();
                    self.inventory.add_item(item)?;
                }
                Ok(())
            }
            Err(e) => Err(InventoryError::EquipError(e)),
        }
    }

    pub fn unequip_item(&mut self, slot: &str) -> Result<(), InventoryError> {
        let equipment_slot = self.equipment.get_mut(slot)
            .ok_or(InventoryError::InvalidSlot)?;

        if let Some(item_rc) = equipment_slot.unequip() {
            let item = Rc::try_unwrap(item_rc)
                .map_err(|_| InventoryError::EquipError("Failed to unequip item".to_string()))?
                .into_inner();
            self.inventory.add_item(item)?;
        }
        Ok(())
    }

    pub fn get_stats(&self) -> String {
        let stats = self.stats.borrow();
        format!(
            "Player: {}\nLevel: {}\nStrength: {}\nDefense: {}",
            self.name, stats.level, stats.strength, stats.defense
        )
    }

    pub fn level_up(&self) {
        let mut stats = self.stats.borrow_mut();
        stats.level += 1;
        stats.strength += 2;
        stats.defense += 2;
    }

    pub fn get_equipped_items(&self) -> Vec<String> {
        self.equipment.iter()
            .map(|(_, slot)| slot.get_equipped_info())
            .collect()
    }

    pub fn process_inventory_async(&self) -> thread::JoinHandle<()> {
        let inventory_items = Arc::clone(&self.inventory.items);
        thread::spawn(move || {
            let items = inventory_items.lock().unwrap();
            for (id, item) in items.iter() {
                println!("Processing item {}: {}", id, item.name);
                // Simulate some processing time
                thread::sleep(std::time::Duration::from_millis(100));
            }
        })
    }
}
