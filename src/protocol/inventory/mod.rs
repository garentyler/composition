pub mod slot;

use slot::Slot;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum InventoryKind {
    Generic9x1 = 0,
    Generic9x2 = 1,
    #[default]
    Generic9x3 = 2,
    Generic9x4 = 3,
    Generic9x5 = 4,
    Generic9x6 = 5,
    Generic3x3 = 6,
    Anvil = 7,
    Beacon = 8,
    BlastFurnace = 9,
    BrewingStand = 10,
    Crafting = 11,
    Enchantment = 12,
    Furnace = 13,
    Grindstone = 14,
    Hopper = 15,
    Lectern = 16,
    Loom = 17,
    Merchant = 18,
    ShulkerBox = 19,
    #[cfg(not(feature = "update_1_20"))]
    LegacySmithing = 20,
    #[cfg(feature = "update_1_20")]
    Smithing = 21,
    Smoker = 22,
    Cartography = 23,
    Stonecutter = 24,
}
impl InventoryKind {
    pub fn as_minecraft_id(&self) -> &'static str {
        match self {
            InventoryKind::Generic9x1 => "minecraft:generic_9x1",
            InventoryKind::Generic9x2 => "minecraft:Generic_9x2",
            InventoryKind::Generic9x3 => "minecraft:Generic_9x3",
            InventoryKind::Generic9x4 => "minecraft:Generic_9x4",
            InventoryKind::Generic9x5 => "minecraft:Generic_9x5",
            InventoryKind::Generic9x6 => "minecraft:Generic_9x6",
            InventoryKind::Generic3x3 => "minecraft:Generic_3x3",
            InventoryKind::Anvil => "minecraft:anvil",
            InventoryKind::Beacon => "minecraft:beacon",
            InventoryKind::BlastFurnace => "minecraft:blast_furnace",
            InventoryKind::BrewingStand => "minecraft:brewing_stand",
            InventoryKind::Crafting => "minecraft:crafting",
            InventoryKind::Enchantment => "minecraft:enchantment",
            InventoryKind::Furnace => "minecraft:furnace",
            InventoryKind::Grindstone => "minecraft:grindstone",
            InventoryKind::Hopper => "minecraft:hopper",
            InventoryKind::Lectern => "minecraft:lectern",
            InventoryKind::Loom => "minecraft:loom",
            InventoryKind::Merchant => "minecraft:merchant",
            InventoryKind::ShulkerBox => "mincraft:shulker_box",
            #[cfg(not(feature = "update_1_20"))]
            InventoryKind::LegacySmithing => "minecraft:legacy_smithing",
            #[cfg(feature = "update_1_20")]
            InventoryKind::Smithing => "minecraft:smithing",
            InventoryKind::Smoker => "minecraft:smoker",
            InventoryKind::Cartography => "minecraft:cartography",
            InventoryKind::Stonecutter => "minecraft:stonecutter",
        }
    }
}

pub struct PlayerInventory([Slot; 46]);
impl PlayerInventory {
    pub fn crafting_output(&self) -> &Slot {
        &self.0[0]
    }
    pub fn crafting_output_mut(&mut self) -> &mut Slot {
        &mut self.0[0]
    }

    pub fn crafting_input(&self) -> &[Slot; 4] {
        <&[Slot; 4]>::try_from(&self.0[1..5]).unwrap()
    }
    pub fn crafting_input_mut(&mut self) -> &mut [Slot; 4] {
        <&mut [Slot; 4]>::try_from(&mut self.0[1..5]).unwrap()
    }

    pub fn head(&self) -> &Slot {
        &self.0[5]
    }
    pub fn head_mut(&mut self) -> &mut Slot {
        &mut self.0[5]
    }
    pub fn chest(&self) -> &Slot {
        &self.0[6]
    }
    pub fn chest_mut(&mut self) -> &mut Slot {
        &mut self.0[6]
    }
    pub fn legs(&self) -> &Slot {
        &self.0[7]
    }
    pub fn legs_mut(&mut self) -> &mut Slot {
        &mut self.0[7]
    }
    pub fn feet(&self) -> &Slot {
        &self.0[8]
    }
    pub fn feet_mut(&mut self) -> &mut Slot {
        &mut self.0[8]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[9..36]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[9..36]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[36..45]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[36..45]).unwrap()
    }

    pub fn offhand(&self) -> &Slot {
        &self.0[45]
    }
    pub fn offhand_mut(&mut self) -> &mut Slot {
        &mut self.0[45]
    }
}

pub struct Chest([Slot; 63]);
impl Chest {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Generic9x3;

    pub fn contents(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[0..27]).unwrap()
    }
    pub fn contents_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[0..27]).unwrap()
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[27..54]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[27..54]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[54..63]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[54..63]).unwrap()
    }
}

pub struct LargeChest([Slot; 90]);
impl LargeChest {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Generic9x6;

    pub fn contents(&self) -> &[Slot; 54] {
        <&[Slot; 54]>::try_from(&self.0[0..54]).unwrap()
    }
    pub fn contents_mut(&mut self) -> &mut [Slot; 54] {
        <&mut [Slot; 54]>::try_from(&mut self.0[0..54]).unwrap()
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[54..81]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[54..81]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[81..90]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[81..90]).unwrap()
    }
}

pub struct CraftingTable([Slot; 46]);
impl CraftingTable {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Crafting;

    pub fn output(&self) -> &Slot {
        &self.0[0]
    }
    pub fn output_mut(&mut self) -> &mut Slot {
        &mut self.0[0]
    }

    pub fn input(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[1..10]).unwrap()
    }
    pub fn input_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[1..10]).unwrap()
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[10..37]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[10..37]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[37..46]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[37..46]).unwrap()
    }
}

pub struct Furnace([Slot; 39]);
impl Furnace {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Furnace;

    pub fn ingredient(&self) -> &Slot {
        &self.0[0]
    }
    pub fn ingredient_mut(&mut self) -> &mut Slot {
        &mut self.0[0]
    }

    pub fn fuel(&self) -> &Slot {
        &self.0[1]
    }
    pub fn fuel_mut(&mut self) -> &mut Slot {
        &mut self.0[1]
    }

    pub fn output(&self) -> &Slot {
        &self.0[2]
    }
    pub fn output_mut(&mut self) -> &mut Slot {
        &mut self.0[2]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[3..30]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[3..30]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[30..39]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[30..39]).unwrap()
    }
}

pub struct BlastFurnace([Slot; 39]);
impl BlastFurnace {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::BlastFurnace;

    pub fn ingredient(&self) -> &Slot {
        &self.0[0]
    }
    pub fn ingredient_mut(&mut self) -> &mut Slot {
        &mut self.0[0]
    }

    pub fn fuel(&self) -> &Slot {
        &self.0[1]
    }
    pub fn fuel_mut(&mut self) -> &mut Slot {
        &mut self.0[1]
    }

    pub fn output(&self) -> &Slot {
        &self.0[2]
    }
    pub fn output_mut(&mut self) -> &mut Slot {
        &mut self.0[2]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[3..30]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[3..30]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[30..39]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[30..39]).unwrap()
    }
}

pub struct Smoker([Slot; 39]);
impl Smoker {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Smoker;

    pub fn ingredient(&self) -> &Slot {
        &self.0[0]
    }
    pub fn ingredient_mut(&mut self) -> &mut Slot {
        &mut self.0[0]
    }

    pub fn fuel(&self) -> &Slot {
        &self.0[1]
    }
    pub fn fuel_mut(&mut self) -> &mut Slot {
        &mut self.0[1]
    }

    pub fn output(&self) -> &Slot {
        &self.0[2]
    }
    pub fn output_mut(&mut self) -> &mut Slot {
        &mut self.0[2]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[3..30]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[3..30]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[30..39]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[30..39]).unwrap()
    }
}

pub struct Dispenser([Slot; 45]);
impl Dispenser {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Generic3x3;

    pub fn contents(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[0..9]).unwrap()
    }
    pub fn contents_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[0..9]).unwrap()
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[9..35]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[9..35]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[36..45]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[36..45]).unwrap()
    }
}

pub struct EnchantmentTable([Slot; 38]);
impl EnchantmentTable {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Enchantment;

    pub fn item(&self) -> &Slot {
        &self.0[0]
    }
    pub fn item_mut(&mut self) -> &mut Slot {
        &mut self.0[0]
    }

    pub fn lapis_lazuli(&self) -> &Slot {
        &self.0[1]
    }
    pub fn lapis_lazuli_mut(&mut self) -> &mut Slot {
        &mut self.0[1]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[2..29]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[2..29]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[29..38]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[29..38]).unwrap()
    }
}

pub struct BrewingStand([Slot; 41]);
impl BrewingStand {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::BrewingStand;

    pub fn bottles(&self) -> &[Slot; 3] {
        <&[Slot; 3]>::try_from(&self.0[0..3]).unwrap()
    }
    pub fn bottles_mut(&mut self) -> &mut [Slot; 3] {
        <&mut [Slot; 3]>::try_from(&mut self.0[0..3]).unwrap()
    }

    pub fn ingredient(&self) -> &Slot {
        &self.0[3]
    }
    pub fn ingredient_mut(&mut self) -> &mut Slot {
        &mut self.0[3]
    }

    pub fn blaze_powder(&self) -> &Slot {
        &self.0[4]
    }
    pub fn blaze_powder_mut(&mut self) -> &mut Slot {
        &mut self.0[4]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[5..32]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[5..32]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[32..41]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[32..41]).unwrap()
    }
}

pub struct VillagerTrading([Slot; 39]);
impl VillagerTrading {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Merchant;

    pub fn input(&self) -> &[Slot; 2] {
        <&[Slot; 2]>::try_from(&self.0[0..2]).unwrap()
    }
    pub fn input_mut(&mut self) -> &mut [Slot; 2] {
        <&mut [Slot; 2]>::try_from(&mut self.0[0..2]).unwrap()
    }

    pub fn result(&self) -> &Slot {
        &self.0[2]
    }
    pub fn result_mut(&mut self) -> &mut Slot {
        &mut self.0[2]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[3..30]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[3..30]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[30..39]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[30..39]).unwrap()
    }
}

pub struct Beacon([Slot; 37]);
impl Beacon {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Beacon;

    pub fn payment(&self) -> &Slot {
        &self.0[0]
    }
    pub fn payment_mut(&mut self) -> &mut Slot {
        &mut self.0[0]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[1..28]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[1..28]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[28..37]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[28..37]).unwrap()
    }
}

pub struct Anvil([Slot; 39]);
impl Anvil {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Anvil;

    pub fn input(&self) -> &[Slot; 2] {
        <&[Slot; 2]>::try_from(&self.0[0..2]).unwrap()
    }
    pub fn input_mut(&mut self) -> &mut [Slot; 2] {
        <&mut [Slot; 2]>::try_from(&mut self.0[0..2]).unwrap()
    }

    pub fn output(&self) -> &Slot {
        &self.0[2]
    }
    pub fn output_mut(&mut self) -> &mut Slot {
        &mut self.0[2]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[3..30]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[3..30]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[30..39]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[30..39]).unwrap()
    }
}

pub struct Hopper([Slot; 41]);
impl Hopper {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Hopper;

    pub fn contents(&self) -> &[Slot; 5] {
        <&[Slot; 5]>::try_from(&self.0[0..5]).unwrap()
    }
    pub fn contents_mut(&mut self) -> &mut [Slot; 5] {
        <&mut [Slot; 5]>::try_from(&mut self.0[0..5]).unwrap()
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[5..32]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[5..32]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[32..41]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[32..41]).unwrap()
    }
}

pub struct Shulker([Slot; 63]);
impl Shulker {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::ShulkerBox;

    pub fn contents(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[0..27]).unwrap()
    }
    pub fn contents_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[0..27]).unwrap()
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[27..54]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[27..54]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[54..63]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[54..63]).unwrap()
    }
}

pub enum Llama {
    Unchested([Slot; 38]),
    Strength1([Slot; 41]),
    Strength2([Slot; 44]),
    Strength3([Slot; 47]),
    Strength4([Slot; 50]),
    Strength5([Slot; 53]),
}
impl Llama {
    pub fn saddle(&self) -> &Slot {
        match self {
            Llama::Unchested(slots) => &slots[0],
            Llama::Strength1(slots) => &slots[0],
            Llama::Strength2(slots) => &slots[0],
            Llama::Strength3(slots) => &slots[0],
            Llama::Strength4(slots) => &slots[0],
            Llama::Strength5(slots) => &slots[0],
        }
    }
    pub fn saddle_mut(&mut self) -> &mut Slot {
        match self {
            Llama::Unchested(slots) => &mut slots[0],
            Llama::Strength1(slots) => &mut slots[0],
            Llama::Strength2(slots) => &mut slots[0],
            Llama::Strength3(slots) => &mut slots[0],
            Llama::Strength4(slots) => &mut slots[0],
            Llama::Strength5(slots) => &mut slots[0],
        }
    }

    pub fn carpet(&self) -> &Slot {
        match self {
            Llama::Unchested(slots) => &slots[1],
            Llama::Strength1(slots) => &slots[1],
            Llama::Strength2(slots) => &slots[1],
            Llama::Strength3(slots) => &slots[1],
            Llama::Strength4(slots) => &slots[1],
            Llama::Strength5(slots) => &slots[1],
        }
    }
    pub fn carpet_mut(&mut self) -> &mut Slot {
        match self {
            Llama::Unchested(slots) => &mut slots[1],
            Llama::Strength1(slots) => &mut slots[1],
            Llama::Strength2(slots) => &mut slots[1],
            Llama::Strength3(slots) => &mut slots[1],
            Llama::Strength4(slots) => &mut slots[1],
            Llama::Strength5(slots) => &mut slots[1],
        }
    }

    pub fn llama_inventory_size(&self) -> usize {
        match self {
            Llama::Unchested(_) => 0,
            Llama::Strength1(_) => 3,
            Llama::Strength2(_) => 6,
            Llama::Strength3(_) => 9,
            Llama::Strength4(_) => 12,
            Llama::Strength5(_) => 15,
        }
    }
    fn llama_inventory_range(&self) -> std::ops::Range<usize> {
        2..(self.llama_inventory_size() + 2)
    }
    fn main_inventory_range(&self) -> std::ops::Range<usize> {
        let start = self.llama_inventory_range().end;
        let end = start + 27;
        start..end
    }
    fn hotbar_range(&self) -> std::ops::Range<usize> {
        let start = self.main_inventory_range().end;
        let end = start + 9;
        start..end
    }

    pub fn llama_inventory(&self) -> &[Slot] {
        let r = self.llama_inventory_range();
        match self {
            Llama::Unchested(_) => &[],
            Llama::Strength1(slots) => &slots[r],
            Llama::Strength2(slots) => &slots[r],
            Llama::Strength3(slots) => &slots[r],
            Llama::Strength4(slots) => &slots[r],
            Llama::Strength5(slots) => &slots[r],
        }
    }
    pub fn llama_inventory_mut(&mut self) -> &mut [Slot] {
        let r = self.llama_inventory_range();
        match self {
            Llama::Unchested(_) => &mut [],
            Llama::Strength1(slots) => &mut slots[r],
            Llama::Strength2(slots) => &mut slots[r],
            Llama::Strength3(slots) => &mut slots[r],
            Llama::Strength4(slots) => &mut slots[r],
            Llama::Strength5(slots) => &mut slots[r],
        }
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        let r = self.main_inventory_range();
        let s = match self {
            Llama::Unchested(_) => &[],
            Llama::Strength1(slots) => &slots[r],
            Llama::Strength2(slots) => &slots[r],
            Llama::Strength3(slots) => &slots[r],
            Llama::Strength4(slots) => &slots[r],
            Llama::Strength5(slots) => &slots[r],
        };
        <&[Slot; 27]>::try_from(s).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        let r = self.main_inventory_range();
        let s = match self {
            Llama::Unchested(_) => &mut [],
            Llama::Strength1(slots) => &mut slots[r],
            Llama::Strength2(slots) => &mut slots[r],
            Llama::Strength3(slots) => &mut slots[r],
            Llama::Strength4(slots) => &mut slots[r],
            Llama::Strength5(slots) => &mut slots[r],
        };
        <&mut [Slot; 27]>::try_from(s).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        let r = self.hotbar_range();
        let s = match self {
            Llama::Unchested(_) => &[],
            Llama::Strength1(slots) => &slots[r],
            Llama::Strength2(slots) => &slots[r],
            Llama::Strength3(slots) => &slots[r],
            Llama::Strength4(slots) => &slots[r],
            Llama::Strength5(slots) => &slots[r],
        };
        <&[Slot; 9]>::try_from(s).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        let r = self.hotbar_range();
        let s = match self {
            Llama::Unchested(_) => &mut [],
            Llama::Strength1(slots) => &mut slots[r],
            Llama::Strength2(slots) => &mut slots[r],
            Llama::Strength3(slots) => &mut slots[r],
            Llama::Strength4(slots) => &mut slots[r],
            Llama::Strength5(slots) => &mut slots[r],
        };
        <&mut [Slot; 9]>::try_from(s).unwrap()
    }
}

pub struct Horse([Slot; 38]);
impl Horse {
    pub fn saddle(&self) -> &Slot {
        &self.0[0]
    }
    pub fn saddle_mut(&mut self) -> &mut Slot {
        &mut self.0[0]
    }

    pub fn armor(&self) -> &Slot {
        &self.0[1]
    }
    pub fn armor_mut(&mut self) -> &mut Slot {
        &mut self.0[1]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[2..29]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[2..29]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[29..38]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[29..38]).unwrap()
    }
}

pub enum Donkey {
    Unchested([Slot; 38]),
    Chested([Slot; 53]),
}
impl Donkey {
    pub fn saddle(&self) -> &Slot {
        match self {
            Donkey::Unchested(slots) => &slots[0],
            Donkey::Chested(slots) => &slots[0],
        }
    }
    pub fn saddle_mut(&mut self) -> &mut Slot {
        match self {
            Donkey::Unchested(slots) => &mut slots[0],
            Donkey::Chested(slots) => &mut slots[0],
        }
    }

    pub fn armor(&self) -> &Slot {
        match self {
            Donkey::Unchested(slots) => &slots[1],
            Donkey::Chested(slots) => &slots[1],
        }
    }
    pub fn armor_mut(&mut self) -> &mut Slot {
        match self {
            Donkey::Unchested(slots) => &mut slots[1],
            Donkey::Chested(slots) => &mut slots[1],
        }
    }

    pub fn donkey_inventory(&self) -> Option<&[Slot; 15]> {
        if let Donkey::Chested(slots) = self {
            Some(<&[Slot; 15]>::try_from(&slots[2..17]).unwrap())
        } else {
            None
        }
    }
    pub fn donkey_inventory_mut(&mut self) -> Option<&mut [Slot; 15]> {
        if let Donkey::Chested(slots) = self {
            Some(<&mut [Slot; 15]>::try_from(&mut slots[2..17]).unwrap())
        } else {
            None
        }
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        let s = match self {
            Donkey::Unchested(slots) => &slots[2..29],
            Donkey::Chested(slots) => &slots[17..44],
        };
        <&[Slot; 27]>::try_from(s).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        let s = match self {
            Donkey::Unchested(slots) => &mut slots[2..29],
            Donkey::Chested(slots) => &mut slots[17..44],
        };
        <&mut [Slot; 27]>::try_from(s).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        let s = match self {
            Donkey::Unchested(slots) => &slots[29..38],
            Donkey::Chested(slots) => &slots[44..53],
        };
        <&[Slot; 9]>::try_from(s).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        let s = match self {
            Donkey::Unchested(slots) => &mut slots[29..38],
            Donkey::Chested(slots) => &mut slots[44..53],
        };
        <&mut [Slot; 9]>::try_from(s).unwrap()
    }
}

pub struct CartographyTable([Slot; 39]);
impl CartographyTable {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Cartography;

    pub fn map(&self) -> &Slot {
        &self.0[0]
    }
    pub fn map_mut(&mut self) -> &mut Slot {
        &mut self.0[0]
    }

    pub fn paper(&self) -> &Slot {
        &self.0[1]
    }
    pub fn paper_mut(&mut self) -> &mut Slot {
        &mut self.0[1]
    }

    pub fn output(&self) -> &Slot {
        &self.0[2]
    }
    pub fn output_mut(&mut self) -> &mut Slot {
        &mut self.0[2]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[3..30]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[3..30]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[30..39]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[30..39]).unwrap()
    }
}

pub struct Grindstone([Slot; 39]);
impl Grindstone {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Grindstone;

    pub fn input(&self) -> &[Slot; 2] {
        <&[Slot; 2]>::try_from(&self.0[0..2]).unwrap()
    }
    pub fn input_mut(&mut self) -> &mut [Slot; 2] {
        <&mut [Slot; 2]>::try_from(&mut self.0[0..2]).unwrap()
    }

    pub fn output(&self) -> &Slot {
        &self.0[2]
    }
    pub fn output_mut(&mut self) -> &mut Slot {
        &mut self.0[2]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[3..30]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[3..30]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[30..39]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[30..39]).unwrap()
    }
}

pub struct Lectern(Slot);
impl Lectern {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Lectern;

    pub fn book(&self) -> &Slot {
        &self.0
    }
    pub fn book_mut(&mut self) -> &mut Slot {
        &mut self.0
    }
}

pub struct Loom([Slot; 40]);
impl Loom {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Loom;

    pub fn banner(&self) -> &Slot {
        &self.0[0]
    }
    pub fn banner_mut(&mut self) -> &mut Slot {
        &mut self.0[0]
    }

    pub fn dye(&self) -> &Slot {
        &self.0[1]
    }
    pub fn dye_mut(&mut self) -> &mut Slot {
        &mut self.0[1]
    }

    pub fn pattern(&self) -> &Slot {
        &self.0[2]
    }
    pub fn pattern_mut(&mut self) -> &mut Slot {
        &mut self.0[2]
    }

    pub fn output(&self) -> &Slot {
        &self.0[3]
    }
    pub fn output_mut(&mut self) -> &mut Slot {
        &mut self.0[3]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[4..31]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[4..31]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[31..40]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[31..40]).unwrap()
    }
}

pub struct Stonecutter([Slot; 38]);
impl Stonecutter {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Stonecutter;

    pub fn input(&self) -> &Slot {
        &self.0[0]
    }
    pub fn input_mut(&mut self) -> &mut Slot {
        &mut self.0[0]
    }

    pub fn output(&self) -> &Slot {
        &self.0[1]
    }
    pub fn output_mut(&mut self) -> &mut Slot {
        &mut self.0[1]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[2..29]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[2..29]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[29..38]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[29..38]).unwrap()
    }
}

#[cfg(not(feature = "update_1_20"))]
pub struct LegacySmithingTable([Slot; 39]);
#[cfg(not(feature = "update_1_20"))]
impl LegacySmithingTable {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::LegacySmithing;

    pub fn base_item(&self) -> &Slot {
        &self.0[0]
    }
    pub fn base_item_mut(&mut self) -> &mut Slot {
        &mut self.0[0]
    }

    pub fn additional_item(&self) -> &Slot {
        &self.0[1]
    }
    pub fn additional_item_mut(&mut self) -> &mut Slot {
        &mut self.0[1]
    }

    pub fn output(&self) -> &Slot {
        &self.0[2]
    }
    pub fn output_mut(&mut self) -> &mut Slot {
        &mut self.0[2]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[3..30]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[3..30]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[30..39]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[30..39]).unwrap()
    }
}

#[cfg(feature = "update_1_20")]
pub struct SmithingTable([Slot; 40]);
#[cfg(feature = "update_1_20")]
impl SmithingTable {
    pub const INVENTORY_KIND: InventoryKind = InventoryKind::Smithing;

    pub fn template(&self) -> &Slot {
        &self.0[0]
    }
    pub fn template_mut(&mut self) -> &mut Slot {
        &mut self.0[0]
    }

    pub fn base_item(&self) -> &Slot {
        &self.0[1]
    }
    pub fn base_item_mut(&mut self) -> &mut Slot {
        &mut self.0[1]
    }

    pub fn additional_item(&self) -> &Slot {
        &self.0[2]
    }
    pub fn additional_item_mut(&mut self) -> &mut Slot {
        &mut self.0[2]
    }

    pub fn output(&self) -> &Slot {
        &self.0[3]
    }
    pub fn output_mut(&mut self) -> &mut Slot {
        &mut self.0[3]
    }

    pub fn main_inventory(&self) -> &[Slot; 27] {
        <&[Slot; 27]>::try_from(&self.0[4..31]).unwrap()
    }
    pub fn main_inventory_mut(&mut self) -> &mut [Slot; 27] {
        <&mut [Slot; 27]>::try_from(&mut self.0[4..31]).unwrap()
    }

    pub fn hotbar(&self) -> &[Slot; 9] {
        <&[Slot; 9]>::try_from(&self.0[31..40]).unwrap()
    }
    pub fn hotbar_mut(&mut self) -> &mut [Slot; 9] {
        <&mut [Slot; 9]>::try_from(&mut self.0[31..40]).unwrap()
    }
}
