use bevy::{platform::collections::HashMap, prelude::*};
use crate::item::ItemType;

#[derive(Resource, Debug, Default)]
pub struct PlayerInventory {
    pub items: HashMap<ItemType, u32>,
} 
impl PlayerInventory {
    pub fn add(
        &mut self,
        item: ItemType,
        count: u32,
    ) {
        *self.items.entry(item).or_default() += count;
    }

    pub fn has_amount(
        &self, 
        item: ItemType, 
        count: u32
    ) -> bool {
        return self.items.get(&item).copied().unwrap_or(0) >= count;
    }

    pub fn try_remove(
        &mut self, 
        item: ItemType, 
        count: u32
    ) -> bool {
        if !self.has_amount(item, count) {
            return false;
        } 

        if let Some(current_count) = self.items.get_mut(&item) {
            *current_count -= count;
            // Clean up empty keys to keep your UI loops tight and neat
            if *current_count == 0 {
                self.items.remove(&item);
            }
        }
        true
    }
}

#[derive(Resource)]
pub struct SelectedItem(pub ItemType);
impl Default for SelectedItem {
    fn default() -> Self {
        Self(ItemType::IronOre) //TODO: Fix
    }
}



