
use tron_net::types::{from_base58, from_hex, Address};

fn main() {
    let from = "TJnGao5hYkJxSxs2vm7orE5pxHQf2ofQQU";
    let address1 = Address::from_private_key("57987740a1a38722d08deea2bd62717019781f07acf38e0eda9c0ba0e2b27021");
    assert_eq!(address1.to_base58(),"TGGhcNeS3cDGJUSUjLDJBQSVUjAM57vizg");
    // let address2 = Address::from_base58(from);
    // let a = address2.to_base58();
    // assert_eq!(address, a);
}