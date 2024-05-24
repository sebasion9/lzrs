#[derive(Debug, PartialEq)]
pub struct Pair { offset : usize, len : usize }
impl Pair { pub fn new(offset : usize, len : usize) -> Self {  Self { offset, len } }}
pub struct LZSS {
    hist_size : u32,
    win_size : u32
}
// aabbcabbcabd. 
#[allow(warnings)]
impl LZSS {
    pub fn new(hist_size : u32, win_size : u32) -> Self { Self { hist_size, win_size } }

    pub fn compress(&self, input : Vec<u8>) {

        // SLIDING WINDOW && HISTORY WINDOW
        // IF left < hist_size, -> tmp buffer containing input[0] and lookup there
        // ELSE his win is just left - hist_size, so input[left - hist_size]..input[left]


        let mut tmp_his_buf : Vec<u8> = self.init_tmp_hisbuf(input[0]);
        let mut right = self.win_size as usize;
        for mut left in 0..(input.len() - right + 1) {

            // SLIDING WIN
            let win_slice : &[u8] = &input[left..right];

            // HIST WIN
            let mut his_slice : &[u8];
            if left < self.hist_size as usize {
                if left != 0 {
                    LZSS::buf_shift_left_and_insert(&mut tmp_his_buf, vec![input[left - 1]]);
                }
                his_slice = &tmp_his_buf;
            }
            else {
                his_slice = &input[left - self.hist_size as usize..left];
            }

            // DBG

            println!("step: {}, history: {:?}, sliding: {:?}",left, 
                     his_slice
                     .iter()
                     .map(|c| { *c as char })
                     .collect::<Vec<char>>(),
                     win_slice
                     .iter()
                     .map(|c| { *c as char })
                     .collect::<Vec<char>>());
            right += 1;
        }
    }

    pub fn decompress(&self, input : Vec<u8>) {

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
            for j in 0..sub.len() {
                let mut offset = 0;
                'inner: while input[i + offset] == sub[j + offset] {
                    offset += 1;
                    if offset > max {
                        max = offset;
                        idx = i;
                    }
                    if offset + i >= input.len() || offset + j >= sub.len() {
                        break 'inner;
                    }
                }
            }
        }
        if max > 0 {
            return Some(Pair::new(idx, max))
        }
        return None
    }
}











