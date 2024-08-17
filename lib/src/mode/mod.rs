mod command;
mod insert;
mod normal;

pub use self::command::*;
pub use self::insert::*;
pub use self::normal::*;

#[derive(Debug, Default, Clone, Copy)]
pub enum VimMode {
    #[default]
    Normal,
    Insert,
    Visual,
}
