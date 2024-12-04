use crate::protocol::mctypes::VarInt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Sniffer {
    pub state: SnifferState,
    pub seed_drop_ticks: VarInt,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum SnifferState {
    #[default]
    Idling = 0,
    FeelingHappy = 1,
    Scenting = 2,
    Sniffing = 3,
    Searching = 4,
    Digging = 5,
    Rising = 6,
}
