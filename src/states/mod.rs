pub mod menu;
pub mod gameplay;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    Gameplay,
}