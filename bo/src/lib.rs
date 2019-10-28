mod bau;
mod miao;
// this exposes `cheers` via `crate::argh`
pub mod argh;

pub use bau::lol::prant;
pub use miao::prunt;
// this exposes `cheers` via `crate`
pub use argh::cheers;

pub fn print() {
    println!("I am BO");
}
