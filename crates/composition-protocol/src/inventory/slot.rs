#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Slot {
    pub contents: Option<ItemStack>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct ItemStack {
    // TODO: Item ID
    pub id: (),
    pub count: u8,
    // TODO: NBT
    pub nbt: (),
}
