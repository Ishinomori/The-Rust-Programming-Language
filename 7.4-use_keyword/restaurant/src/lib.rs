use std::cmp::Ordering;
use std::io;
// equal to
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
// equal to
use std::io::{self, Write};

// all
use std::collections::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}