#[derive(Debug, PartialEq)]
pub struct Pair { offset : usize, len : usize }
impl Pair { 
    pub fn new(offset : usize, len : usize) -> Self {
        Self { offset, len } 
    }
}
pub struct LZSS {
    hist_size : u32,
    win_size : u32,
    hist_bitlen : u32,
    win_bitlen : u32
}
#[derive(Debug)]
pub enum Token {
    Byte(u8),
    Pair(Pair)
}

// aabbcabbcabd. 
#[allow(warnings)]
impl LZSS {
    pub fn new(hist_size : u32, win_size : u32) -> Self { 
        let hist_bitlen = (hist_size as f32).log2() as u32 + 1;
        let win_bitlen = (win_size as f32).log2() as u32 + 1;
        Self { hist_size, win_size, hist_bitlen, win_bitlen} 
    }

    pub fn compress(&self, input : Vec<u8>) -> Vec<Token> {

        // SLIDING WINDOW && HISTORY WINDOW
        // IF left < hist_size, -> tmp buffer containing input[0] and lookup there
        // ELSE his win is just left - hist_size, so input[left - hist_size]..input[left]

        println!("input: {:?}", input
                 .iter()
                 .map(|c| {*c as char})
                 .collect::<Vec<char>>());

        let mut tmp_his_buf : Vec<u8> = self.init_tmp_hisbuf(input[0]);
        let mut right = self.win_size as usize;
        let mut len = 1;
        let mut left = 0;
        let mut tokens : Vec<Token> = vec![];
        while left < (input.len()) {
            // SLIDING WIN
            let win_slice : &[u8] = &input[left..right];
            let dbg_idx = left;

            // HIST WIN
            let mut his_slice : &[u8];
            if left < self.hist_size as usize {
                if left != 0 {
                    let shift_slice = &input[(left - len)..left];
                    LZSS::buf_shift_left_and_insert(&mut tmp_his_buf, shift_slice.to_vec());
                }
                his_slice = &tmp_his_buf;
            }
            else {
                his_slice = &input[left - self.hist_size as usize..left];
            }

            if let Some(pair) = LZSS::max_sub(his_slice, win_slice) {
                len = pair.len;
                tokens.push(Token::Pair(pair));
            }
            else {
                len = 1;
                tokens.push(Token::Byte(input[left]));
            }
            left += len;
            right = left + self.win_size as usize;
            if right > input.len() {
                right = input.len();
            }
            // DBG
            println!("step: {}, history: {:?}, sliding: {:?}",dbg_idx, 
                     his_slice
                     .iter()
                     .map(|c| { *c as char })
                     .collect::<Vec<char>>(),
                     win_slice
                     .iter()
                     .map(|c| { *c as char })
                     .collect::<Vec<char>>())
        }
        tokens


    }
    pub fn token_to_bytes(&self, tokens : Vec<Token>) -> Vec<u8> {
        let token_len = self.token_len_inbytes();
        let mut bytes : Vec<u8> = vec![];
        tokens.iter().for_each(|token| {
            match &token {
                Token::Byte(byte) => {
                    bytes.push((byte >> 1)|(1<<7));
                    bytes.push((1<<7)&(byte << 7));
                }
                Token::Pair(pair) => {
                    bytes.push(b'x');
                }
            }
        });
        // implement method to merge converted bytes
        bytes
    }
    // @TODO
    // second byte
    pub fn merge_bytes(dst : u8, src : u8, offset: u8, srcsize : u8) -> Vec<u8> {
        let mut res = vec![];
        let osh = offset + 1;
        // x >> size - idx+1
        if osh < srcsize {
            let mut a = dst & (!0 << offset);
            a |= (src >> (srcsize - osh));
            res.push(a);
            res.push(0);
            return res;
        }
        let off = osh - srcsize;
        let mut a = dst & (!0 << offset);
        a |= (src << off);

        res.push(a);
        res.push(0);
        res
    }

    pub fn decompress(&self, input : Vec<u8>) {

    }
    pub fn token_len_inbytes(&self) -> u32{
        return (self.hist_bitlen + self.win_bitlen + 1) / 8 + 1;
    }

    fn init_tmp_hisbuf(&self, byte : u8) -> Vec<u8> {
        let mut vec = vec![];
        for i in 0..self.hist_size {
            vec.push(byte);
        }
        vec
    }

    pub fn buf_shift_left_and_insert(vec : &mut Vec<u8>, insert_values : Vec<u8>) -> &mut Vec<u8> {
        for value in insert_values {
            vec.remove(0);
            vec.push(value);
        }
        vec
    }
    pub fn max_sub(input : &[u8], sub : &[u8]) -> Option<Pair> {
        let mut max = 0;
        let mut idx = 0;
        for i in 0..input.len() {
            let mut offset = 0;
            'inner: while input[i + offset] == sub[offset] {
                offset += 1;
                if offset > max {
                    max = offset;
                    idx = i;
                }
                if offset + i >= input.len() || offset >= sub.len() {
                    break 'inner;
                }
            }
        }
        if max > 0 {
            return Some(Pair::new(idx, max))
        }
        return None
    }
}











