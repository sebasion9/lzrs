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
        for left in 0..(input.len() - right + 1) {

            if left < self.hist_size as usize {
                if left != 0 {
                    LZSS::buf_shift_left_and_insert(&mut tmp_his_buf, vec![input[left - 1]]);
                }
                print!("step: {}, history: {:?}", left, String::from_utf8(tmp_his_buf.iter().map(|c| {*c as u8}).collect::<Vec<u8>>()).expect("should be ascii"));
            }
            else {
                print!("step: {}, history: ", left);
                for i in (left - self.hist_size as usize)..left {
                    print!("{} ", input[i] as char);
                }
            }

            let mut ptr = left.clone();
            print!("step: {}, sliding: ", left);
            while ptr < right {
                print!("{} ", input[ptr] as char);
                ptr += 1;
            }
            println!("");
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
}

