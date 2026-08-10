use std::fmt::Display;

pub mod gamerules;
pub mod general;
pub mod perft;
// Games
pub mod ataxx;
pub mod chess;
pub mod connect4;
pub mod droptaxx;
pub mod gomoku;
pub mod isolation;
pub mod pijersi;
pub mod tak;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Games {
    Ataxx,
    Chess,
    Connect4,
    Droptaxx,
    Gomoku,
    Isolation,
    Pijersi,
    Tak,
}

#[must_use]
pub fn parse_game_name(name: &str) -> Option<Games> {
    match name {
        "ataxx" => Some(Games::Ataxx),
        "chess" => Some(Games::Chess),
        "connect4" => Some(Games::Connect4),
        "droptaxx" => Some(Games::Droptaxx),
        "gomoku" => Some(Games::Gomoku),
        "isolation" => Some(Games::Isolation),
        "pijersi" => Some(Games::Pijersi),
        "tak" => Some(Games::Tak),
        _ => None,
    }
}

impl Display for Games {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Games::Ataxx => write!(f, "Ataxx"),
            Games::Chess => write!(f, "Chess"),
            Games::Connect4 => write!(f, "Connect4"),
            Games::Droptaxx => write!(f, "Droptaxx"),
            Games::Gomoku => write!(f, "Gomoku"),
            Games::Isolation => write!(f, "Isolation"),
            Games::Pijersi => write!(f, "Pijersi"),
            Games::Tak => write!(f, "Tak"),
        }
    }
}
