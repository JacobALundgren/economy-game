use std::{io, convert::TryInto};

use crate::visualization::Tab;
pub enum InputAction {
    Quit,
    TogglePause,
    MoveUp,
    MoveDown,
    Decrease,
    Increase,
    SwitchTab(Tab),
}

fn match_tab_hotkey(key: u8) -> Option<Tab> {
    (0..Tab::count())
        .into_iter()
        .map(|i| <_ as TryInto<Tab>>::try_into(i).unwrap())
        .find(|tab| tab.get_hotkey() == key)
}

pub fn parse_input<R: Iterator<Item=Result<u8, io::Error>>>(r: &mut R) -> Option<InputAction> {
    let item = r.next()?.ok()?;
    if let Some(tab) = match_tab_hotkey(item) {
        return Some(InputAction::SwitchTab(tab));
    }
    match item {
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
