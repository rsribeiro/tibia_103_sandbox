/********************************************************************************
 * 
 * Tibia version 1.03 was released on 1997-02-08.
 * The Tibia website archived on 1997-05-13 only the following chat modes.
 * - #W for whispering
 * - #Y for yelling
 * - #B for broadcasting
 * 
 * Reference:
 * https://web.archive.org/web/19970513130635/http://www-wi.uni-regensburg.de/~vos19618/tibia/e_anleitung.html
 * 
 ********************************************************************************/
pub mod encoding;
use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
pub enum ChatType {
    Broadcast   = 0x0041,
    Look        = 0x004D,
    Normal      = 0x0053,
    Whisper     = 0x0057,
    Yell        = 0x0059,
}

impl From<Option<char>> for ChatType {
    fn from(value: Option<char>) -> Self {
        match value {
            Some('w') | Some('W') => ChatType::Whisper,
            Some('y') | Some('Y') => ChatType::Yell,
            Some('b') | Some('B') => ChatType::Broadcast,
            _ => ChatType::Normal,
        }
    }
}