pub mod name_helpers {
    pub fn build_full_name(first: &str, last: &str) -> String {
        let full_name: String = format!("{0} {1}", first.trim(), last.trim());
        full_name
    }
}

pub mod privatefns {
    pub fn get_age_plus_5(age: u16) -> u16 {
        age + 5
    }
}
