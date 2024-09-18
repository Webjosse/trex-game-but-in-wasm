pub enum EventId{
    JumpDownEvent,
    JumpUpEvent,
    SneakDownEvent,
    SneakUpEvent,
    RestartPressEvent,
    Unknown
}

impl EventId{
    pub fn from_keycode_down(key_code:u32) -> &'static [EventId]{
        match key_code {
            32 => &[EventId::JumpDownEvent],
            38 => &[EventId::JumpDownEvent],
            40 => &[EventId::SneakDownEvent],
            _ => &[]
        }
    }

    pub fn from_keycode_up(key_code:u32) -> &'static [EventId]{
        match key_code {
            32 => &[EventId::JumpUpEvent],
            38 => &[EventId::JumpUpEvent],
            40 => &[EventId::SneakUpEvent],
            _ => &[]
        }
    }


    pub fn from_keycode_press(key_code:u32) -> &'static [EventId]{
        match key_code {
            32 => &[EventId::RestartPressEvent],
            _ => &[]
        }
    }

    #[allow(dead_code)]
    pub fn from_int(int: u16) -> Self {
        match int{
            0 => EventId::JumpDownEvent,
            1 => EventId::JumpUpEvent,
            2 => EventId::SneakDownEvent,
            3 => EventId::SneakUpEvent,
            4 => EventId::RestartPressEvent,
            _ => EventId::Unknown
        }
    }

    #[allow(dead_code)]
    pub fn as_int(&self) -> u16 {
        match self{
            &EventId::JumpDownEvent => 0,
            &EventId::JumpUpEvent => 1,
            &EventId::SneakDownEvent => 2,
            &EventId::SneakUpEvent => 3,
            &EventId::RestartPressEvent => 4,
            EventId::Unknown => u16::MAX
        }
    }
}