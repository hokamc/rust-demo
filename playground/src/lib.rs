pub mod house {
    pub mod people {
        pub fn add() {
            println!("lung")
        }
    }
}

pub fn eat() {
    house::people::add();
}
