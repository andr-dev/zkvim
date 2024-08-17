use nom::{
    branch::alt, bytes::complete::tag, character::complete::anychar, combinator::map, IResult,
};

#[derive(Debug)]
pub enum InsertModeAction {
    Char(char),
    Escape,
}

pub fn insert_mode_action(input: &str) -> IResult<&str, InsertModeAction> {
    alt((
        // spacer
        map(tag("<Esc>"), |_| InsertModeAction::Escape),
        map(anychar, |char| InsertModeAction::Char(char)),
    ))(input)
}
