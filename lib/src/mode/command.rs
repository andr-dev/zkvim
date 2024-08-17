use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, combinator::map,
    sequence::preceded, IResult,
};

#[derive(Debug)]
pub enum VimCommand {
    JumpTo { line: usize },
    JumpForward { lines: usize },
    JumpBackward { lines: usize },
}

// TODO(abenedito): u32 is unsafe here, what if number is larger?
pub fn vim_command(input: &str) -> IResult<&str, VimCommand> {
    alt((
        map(u32, |line| VimCommand::JumpTo {
            line: line.try_into().unwrap(),
        }),
        map(preceded(tag("+"), u32), |lines| VimCommand::JumpForward {
            lines: lines.try_into().unwrap(),
        }),
        map(preceded(tag("-"), u32), |lines| VimCommand::JumpBackward {
            lines: lines.try_into().unwrap(),
        }),
    ))(input)
}
