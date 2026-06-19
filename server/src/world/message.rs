use crate::player::Player;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone, Debug)]
pub enum PlayerToWorld {
    Login(Player, UnboundedSender<WorldToPlayer>),
    Logout(Player),
}

#[derive(Clone, Copy, Debug)]
pub enum WorldToPlayer {}