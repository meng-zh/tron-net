
use tron_net::types::{from_base58, from_hex, Address};

fn main() {
    let from = "TJnGao5hYkJxSxs2vm7orE5pxHQf2ofQQU";
    let address2 = Address::from_base58(from);
    let a = address2.to_base58();
    assert_eq!(address, a);
}