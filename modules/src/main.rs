pub mod kitchen {
    pub fn cook() {
        println!("Yummy food!");
    }

    pub fn wash_dishes() {
        println!("Splash splash!");
    }
}

fn main() {
    kitchen::cook();
    kitchen::wash_dishes()
}
