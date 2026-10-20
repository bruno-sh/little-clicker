pub trait ButtonToU8 {
    fn to_u8(&self) -> u8;
}

impl ButtonToU8 for enigo::Button {
    fn to_u8(&self) -> u8 {
        match self {
            // common buttons
            enigo::Button::Left => 1,
            enigo::Button::Middle => 2,
            enigo::Button::Right => 3,

            // unknown buttons
            _ => 0,
        }
    }
}

pub trait U8ToButton {
    fn to_button(&self) -> enigo::Button;
}

impl U8ToButton for u8 {
    fn to_button(&self) -> enigo::Button {
        match self {
            // common buttons
            1 => enigo::Button::Left,
            2 => enigo::Button::Middle,
            3 => enigo::Button::Right,

            // unknown buttons
            _ => enigo::Button::ScrollUp,
        }
    }
}
