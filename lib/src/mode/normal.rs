use super::{vim_command, VimCommand};

use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::terminated, IResult};

#[derive(Debug)]
pub enum NormalModeAction {
    EnterInsertMode,
    EnterVisualMode,

    MoveLeft,
    MoveDown,
    MoveUp,
    MoveRight,

    MoveToEndOfLine,

    Command(VimCommand),
}

pub fn normal_mode_action(input: &str) -> IResult<&str, Box<[NormalModeAction]>> {
    if let Ok((input, _)) = tag::<&str, &str, nom::error::Error<&str>>(":")(input) {
        let (input, command) = terminated(vim_command, tag("\n"))(input)?;

        return Ok((input, Box::new([NormalModeAction::Command(command)])));
    }

    alt((
        // Switch Modes
        map(tag("i"), |_| {
            [NormalModeAction::EnterInsertMode].into_iter().collect()
        }),
        map(tag("v"), |_| {
            [NormalModeAction::EnterVisualMode].into_iter().collect()
        }),
        // Navigation
        map(tag("h"), |_| {
            [NormalModeAction::MoveLeft].into_iter().collect()
        }),
        map(tag("j"), |_| {
            [NormalModeAction::MoveDown].into_iter().collect()
        }),
        map(tag("k"), |_| {
            [NormalModeAction::MoveUp].into_iter().collect()
        }),
        map(tag("l"), |_| {
            [NormalModeAction::MoveRight].into_iter().collect()
        }),
        // Misc
        map(tag("A"), |_| {
            [
                NormalModeAction::MoveToEndOfLine,
                NormalModeAction::EnterInsertMode,
            ]
            .into_iter()
            .collect()
        }),
    ))(input)
}
