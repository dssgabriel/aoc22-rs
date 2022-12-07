#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Item(u8);

impl Item {
    pub(crate) fn priority(self) -> u64 {
        match self {
            Item(b'a'..=b'z') => (self.0 - b'a') as u64 + 1,
            Item(b'A'..=b'Z') => (self.0 - b'A') as u64 + 27,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u8> for Item {
    type Error = color_eyre::Report;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
            _ => Err(color_eyre::eyre::eyre!("{value} is not a valid character")),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}
