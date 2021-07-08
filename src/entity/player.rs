use crate::world::location::Location;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub position: Location,
    pub uuid: Uuid,
    names: (String, Option<String>),
}
impl Player {
    pub fn new() -> Player {
        Player {
            position: Location::zero(),
            uuid: Uuid::nil(),
            names: ("Player".into(), None),
        }
    }
    pub fn username(&self) -> &String {
        &self.names.0
    }
    pub fn username_mut(&mut self) -> &mut String {
        &mut self.names.0
    }
    pub fn display_name(&self) -> &String {
        match &self.names.1 {
            Some(name) => name,
            None => &self.names.0,
        }
    }
    pub fn display_name_mut(&mut self) -> &mut Option<String> {
        &mut self.names.1
    }
}
impl Default for Player {
    fn default() -> Self {
        Player::new()
    }
}
