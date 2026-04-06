// have to import sub packages to get items we want.
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main_subpackages() {
    let red = PrimaryColor::Red;
    let blue = PrimaryColor::Blue;
    mix(red, blue);
}
