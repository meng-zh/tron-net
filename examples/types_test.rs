
use tron_net::types::{from_base58, from_hex, Address};

fn main() {
    let from = "TJnGao5hYkJxSxs2vm7orE5pxHQf2ofQQU";
    // let bytes = from_base58(from).expect("error");
    // let address = from_hex(&bytes,0x41);
    let address2 = Address::from_base58(from);
   let a = address2.to_base58();
    println!("{}",a);
    println!("{}",address2);
    // assert_eq!(address,from);
}