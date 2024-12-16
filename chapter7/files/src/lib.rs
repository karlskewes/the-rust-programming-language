// mod is only required once, it's not an "include" which would be required in
// each file that uses the module.
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
