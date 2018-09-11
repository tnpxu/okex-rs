#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
unused_import_braces)]

#[macro_use]
extern crate error_chain;

extern crate hex;
extern crate reqwest;
extern crate ring;
extern crate serde;
extern crate serde_json;

extern crate tungstenite;
extern crate url;

#[macro_use]
extern crate serde_derive;

pub mod errors;

pub mod model;

pub mod websockets;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
