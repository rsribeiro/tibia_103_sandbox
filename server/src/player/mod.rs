use num_enum::TryFromPrimitive;
use crate::{
    map::position::Position
};
use std::sync::atomic::{AtomicU32, Ordering};


/********************************************************************************
 * 
 * Player struct
 * 
 ********************************************************************************/
#[derive(Clone, Debug)]
pub struct Player {
    pub id:         u32,
    pub name:       String,
    pub password:   String,

    pub real_name:  String,
    pub location:   String,
    pub email:      String,

    pub gender:     Gender,
    pub outfit:     OutfitColors,
    pub position:   Position,
    pub direction:  Direction,
}

impl Player {
    pub fn new(name: &str, position: Position) -> Self {
        static NEXT_ID: AtomicU32 = AtomicU32::new(256);
        Self {
            id:         NEXT_ID.fetch_add(1, Ordering::SeqCst),
            name:       name.to_owned(),
            password:   String::new(),

            real_name:  String::new(),
            location:   String::new(),
            email:      String::new(),

            position,
            outfit:     OutfitColors::default(),
            gender:     Gender::Male,
            direction:  Direction::South,
        }
    }
}


/********************************************************************************
 * 
 * Default direction upon login.
 * 
 ********************************************************************************/
impl Default for Direction {
    fn default() -> Self {
        Direction::South
    }
}


/********************************************************************************
 * 
 * Outfit colors.
 * 
 ********************************************************************************/
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OutfitColors {
    pub head:   u8,
    pub body:   u8,
    pub legs:   u8,
    pub shoes:  u8,
}

impl OutfitColors {
    pub const fn new(head: u8, body: u8, legs: u8, shoes: u8) -> Self {
        Self { head, body, legs, shoes }
    }
}


/********************************************************************************
 * 
 * Default outfit colors.
 * 
 ********************************************************************************/
impl Default for OutfitColors {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}


/********************************************************************************
 * 
 * Enums.
 * 
 ********************************************************************************/
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, TryFromPrimitive)]
pub enum Direction {
    North = 0,
    East  = 1,
    South = 2,
    West  = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
pub enum InventorySlot {
    Helmet    = 1,
    Necklace  = 2,
    Bag       = 3,
    Armor     = 4,
    RightHand = 5,
    LeftHand  = 6,
    Legs      = 7,
    Boots     = 8,
}