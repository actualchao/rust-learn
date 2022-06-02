mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("hello world rustaurant")
        }
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
    mod aa {

        fn test_aa () {

        }

        mod cc {
            super::test_aa();
        }
    }
 
    mod bb {
        aa:: test_aa();
    }