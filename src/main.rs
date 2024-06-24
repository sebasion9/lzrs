mod lzss;
mod test;
use lzss::LZSS;

const EXAMPLE : &[u8] = b"aabbcabbcabd";
const LONGEXAMPLE : &[u8] = b"aabbccaaaabbdddbbbddddfffdddaaaa";
fn main() {
    let lz = LZSS::new(4,4);
    lz.compress(EXAMPLE.to_vec());
    //lz.compress(LONGEXAMPLE.to_vec());
    //lz.compress(b"I AM SAM SAM I AM".to_vec());
    //
    let b1 : u8 = 0b1000_0000;
    let b2 : u8 = 0b0000_0011;

    let res3 = LZSS::merge_bytes(b1, b2, 0, 2);
    let exp_3 : Vec<u8> = vec![0b1000_0001, 0b0000_0000];
}
