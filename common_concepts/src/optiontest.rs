pub fn test_option_type() -> Option<u8> {
    let mut option1: Option<u8> = None;
    option1 = Some(8);
    return option1;
}

pub enum CharacterType {
    Archer,
    Mage,
    Warrior,
}

pub fn test_char_type() -> Option<CharacterType> {
    let mut char_type: Option<CharacterType> = None;
    char_type = Some(CharacterType::Archer);
    return char_type;
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        String::from(match self {
            CharacterType::Archer => "Archer",
            CharacterType::Mage => "Mage",
            CharacterType::Warrior => "Warrior",
        })
    }
}
