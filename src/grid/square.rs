use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Square {
    Initial(u8),
    Filled(u8),
    Empty,
}

impl Square {
    pub fn from_value(value: u8) -> Square {
        if value == 0 {
            Square::Empty
        } else {
            assert!(value <= 9 as u8);
            Square::Filled(value)
        }
    }

    pub fn initial(value: u8) -> Square {
        assert!(value > 0 && value <= 9 as u8);
        Square::Initial(value)
    }

    pub fn is_initial(&self) -> bool {
        match *self {
            Square::Initial(_) => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            Square::Empty => true,
            _ => false,
        }
    }

    pub fn value(&self) -> u8 {
        match *self {
            Square::Initial(value) | Square::Filled(value) => value,
            Square::Empty => 0,
        }
    }
}

impl PartialEq for Square {
    fn eq(&self, other: &Square) -> bool {
        self.value() == other.value()
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Square::Initial(value) => write!(f, "{}", value),
            Square::Filled(value) => write!(f, "{}", value),
            Square::Empty => write!(f, " "),
        }
    }
}
