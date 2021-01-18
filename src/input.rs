use std::io;
pub enum InputAction {
    Quit,
    TogglePause,
    MoveUp,
    MoveDown,
    Decrease,
    Increase,
}

pub fn parse_input<R: Iterator<Item=Result<u8, io::Error>>>(r: &mut R) -> Option<InputAction> {
    let item = r.next()?;
    match item.ok()? {
        b'q' => Some(InputAction::Quit),
        b'p' => Some(InputAction::TogglePause),
        27 => parse_escaped(r),
        _ => None,
    }
}

fn parse_escaped<R: Iterator<Item=Result<u8, io::Error>>>(r: &mut R) -> Option<InputAction> {
    let item = r.next()?;
    if item.ok()? != b'[' {
        return None;
    }
    match r.next()?.ok()? {
        b'A' => Some(InputAction::MoveUp),
        b'B' => Some(InputAction::MoveDown),
        b'C' => Some(InputAction::Increase),
        b'D' => Some(InputAction::Decrease),
        _ => None,
    }
}
