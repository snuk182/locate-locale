extern crate locate_locale;

fn main() {
    println!("My user locale is '{}'!", locate_locale::user());
    println!("My system locale is '{}'!", locate_locale::system());
}
