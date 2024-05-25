mod lzss;
mod test;
use lzss::LZSS;

const EXAMPLE : &[u8] = b"aabbcabbcabd";
const LONGEXAMPLE : &[u8] = b"aabbccaaaabbdddbbbddddfffdddaaaa";
fn main() {
    let lz = LZSS::new(4,4);
    //lz.compress(EXAMPLE.to_vec());
    //lz.compress(LONGEXAMPLE.to_vec());
    lz.compress(b"I AM SAM SAM I AM".to_vec());


}
