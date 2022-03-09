use sha2::{Sha256, Digest};
use std::str;
static ALPHABET: &[u8] = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".as_bytes();

fn init()->Vec<i32>{
    let mut indexes :Vec<i32> =  vec![-1;128];
    for i in 0..ALPHABET.len(){
        let index = ALPHABET[i] as usize;
        indexes[index] = i as i32;
    }
    indexes
}
fn twice_hash(data:&[u8])->Vec<u8> {
    let hash0 = Sha256::digest(data).to_vec();
    Sha256::digest(&hash0).to_vec()
}
fn div_mod58(number:&mut Vec<u8>, start_at:usize)->u8{
    let mut remainder = 0u32;
    for i in start_at.. number.len(){
        let digit256 =  number[i] as u32 & 0xff;
        let temp = remainder * 256 + digit256;
        number[i] =(temp/58) as u8;
        remainder = temp % 58;
    }
    remainder as u8
}

fn div_mod256(number58:&mut Vec<u8>, start_at:usize)->u8{
    let mut remainder = 0u32;
    for i in start_at.. number58.len(){
        let digit58 =  number58[i] as u32 & 0xff;
        let temp = remainder * 58 + digit58;
        number58[i] =(temp/256) as u8;
        remainder = temp % 256;
    }
    remainder as u8
}

fn decode(input: &str) ->Result<Vec<u8>, &str> {
    if input.is_empty(){
        Err("input is empty")
    }else{
        let input = input.as_bytes();
        let indexes = init();
        let mut input58:Vec<u8>= vec![0;input.len()];
        for i in 0..input.len(){
            let c = input[i];
            let d = indexes[c as usize];
             if c < 128 && d > 0{
                input58[i] = d as u8;
            }else{
               return  Err("Illegal character");
             }
        }
        let mut zero_count = 0;
        while zero_count < input58.len() && input58[zero_count] == 0 {
            zero_count+=1;
        }
        let mut temp: Vec<u8> = vec![0;input.len()];
        let mut j = temp.len();
        let mut start_at = zero_count;
        while start_at < input58.len() {
            let m = div_mod256(&mut input58, start_at);
            if input58[start_at] == 0{
                start_at +=1;
            }
            j-=1;
            temp[j] = m;
        }
        while j < temp.len() && temp[j] == 0 {
            j +=1;
        }
        Ok(temp[j - zero_count..].to_vec())
    }
}

fn encode(input: Vec<u8>)->Result<String,&'static str>{
    if input.len() == 0{
        Err("input is empty")
    }else{
        let mut zero_count:i32 = 0;
        while zero_count < input.len() as i32 && input[zero_count as usize] == 0 {
            zero_count +=1;
        }
        let mut input = input.clone();
        let mut temp = vec![0;input.len() * 2];
        let mut j = temp.len();
        let mut start_at = zero_count as usize;
        while  start_at < input.len()  {
            let m = div_mod58(&mut input,start_at);
            if input[start_at] == 0{
                start_at +=1;
            }
            j -=1;
            temp[j] = ALPHABET[m as usize] as u8;
        }
        while j < temp.len() && temp[j] == ALPHABET[0] {
            j+=1;
        }
        zero_count -=1;
        while  zero_count >= 0{
            j -=1;
            temp[j] = ALPHABET[0] as u8;
            zero_count -=1;
        }
        let output = std::string::String::from_utf8(temp[j..].to_vec()).unwrap_or_default();
        Ok(output)
    }
}
pub fn from_base58(address:&str)->Result<Vec<u8>, &str>{
    let decode_check = decode(address).unwrap_or_default();
    if decode_check.len() <= 4 {
        Err("Illegal address")
    }else{
        let decode_data = &decode_check[..decode_check.len() - 4];
        let hash0 = twice_hash(&decode_data);
        let mut valid = true;
        for i in 0..4{
            if hash0[i] != decode_check[decode_data.len() + i]{
                valid = false;
                break;
            }
        }
        if valid{
            Ok(decode_data.to_vec())
        }else{
            Err("Illegal checksum")
        }
    }
}

pub fn from_hex(address:&[u8],prefix:u8)->String{
    let mut address_bytes = vec![prefix];
    address_bytes.extend_from_slice(&address[address.len() - 20..]);
    let hash = twice_hash(&address_bytes);
    address_bytes.extend_from_slice(&hash[..4]);
    encode(address_bytes).unwrap_or_default()
}