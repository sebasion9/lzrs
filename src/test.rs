#[cfg(test)]
mod tests {
    use crate::lzss::{Pair, Token, LZSS};
    #[test]
    fn test_shift_vec_mult_values() {
        let mut vec = vec![0,1,2,3];
        let insert_values = vec![5,5];
        LZSS::buf_shift_left_and_insert(&mut vec, insert_values);

        assert_eq!(vec[0], 2);
        assert_eq!(vec[1], 3);
        assert_eq!(vec[2], 5);
        assert_eq!(vec[3], 5);
        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn test_shift_vec_single_value() {
        let mut vec = vec![0,1,2,3];
        let insert_values = vec![5];
        LZSS::buf_shift_left_and_insert(&mut vec, insert_values);

        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
        assert_eq!(vec[3], 5);
        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn test_longest_subset() {
        let t1 : &[u8] = b"aabbb";
        let t2 : &[u8] = b"bbbbb";

        let t3 : &[u8] = b"aaaabbxxxxxw";
        let t4 : &[u8] = b"aaaaaaaaaabb";

        let t5 : &[u8] = b"zxabcdezy";
        let t6 : &[u8] = b"yzabcdezx";

        let t7: &[u8] = b"abcdxyz";
        let t8: &[u8] = b"xyzabcd";

        let t9: &[u8] = b"hellothere";
        let t10: &[u8] = b"herethehello";

        let t11: &[u8] = b"commonprefix";
        let t12: &[u8] = b"commonpostfix";

        let t13: &[u8] = b"abbc";
        let t14: &[u8] = b"ab";


        let res1 = LZSS::max_sub(&t1, &t2);
        let res2 = LZSS::max_sub(&t3, &t4);
        let res3 = LZSS::max_sub(&t5, &t6);
        let res4 = LZSS::max_sub(&t7, &t8);
        let res5 = LZSS::max_sub(&t9, &t10);
        let res6 = LZSS::max_sub(&t11, &t12);
        let res7 = LZSS::max_sub(&t13, &t14);

        assert_eq!(res1, Some(Pair::new(2, 3)));
        assert_eq!(res2, Some(Pair::new(0, 4)));
        assert_eq!(res3, Some(Pair::new(8,1)));
        assert_eq!(res4, Some(Pair::new(4, 3)));
        assert_eq!(res5, Some(Pair::new(6, 4)));
        assert_eq!(res6, Some(Pair::new(0, 7)));
        assert_eq!(res7, Some(Pair::new(0, 2)));
    }

    #[test]
    fn test_token_len_inbytes() {
        let lzss1 = LZSS::new(4,4);
        let res1 = lzss1.token_len_inbytes();


        let lzss2 = LZSS::new(15,3);
        let res2 = lzss2.token_len_inbytes();

        let lzss3 = LZSS::new(224,129);
        let res3 = lzss3.token_len_inbytes();


        assert_eq!(res1, 1);
        assert_eq!(res2, 1);
        assert_eq!(res3, 3);
    }
    #[test]
    fn test_token_to_bytes() {
        let lzss = LZSS::new(4,4);
        let test_byte = b'a';
        let t1 = Token::Byte(test_byte);
        let res1 = lzss.token_to_bytes(vec![t1]);
        assert_eq!(res1, vec![176 as u8, 128 as u8]);
        // should return 2 bytes [1{7bits of char a}{last bit of char a}]

        let t2 = Token::Byte(b'x');
        let res2 = lzss.token_to_bytes(vec![t2]);
        assert_eq!(res2, vec![188 as u8, 0]);

        todo!();

        let t3 = Token::Pair(Pair::new(2,3));
        let res3 = lzss.token_to_bytes(vec![t3]);
        assert_eq!(res3, vec![]);
        // with parameters 4,4 should return [0{3bits of offset}{3bits of size}]

        let lzss2 = LZSS::new(16,16);
        let t4 = Token::Pair(Pair::new(5,10));
        let res4 = lzss.token_to_bytes(vec![t4]);
        assert_eq!(res4, vec![]);
    }

    #[test]
    fn test_merge_bytes() {
        let b1 : u8 = 0b1000_0000;
        let b2 : u8 = 0b0000_0011;
        let b3 : u8 = 0b0000_1111;


        let res1 = LZSS::merge_bytes(b1, b2, 6, 2);
        let res2 = LZSS::merge_bytes(b1, b2, 1, 2);
        let res3 = LZSS::merge_bytes(b1, b2, 0, 2);
        let res4 = LZSS::merge_bytes(b1, b3, 2, 4);
        let res5 = LZSS::merge_bytes(b1, b3, 0, 4);
        let exp_1 : Vec<u8> = vec![0b1110_0000, 0b0000_0000];
        let exp_2 : Vec<u8> = vec![0b1000_0011, 0b0000_0000];
        let exp_3 : Vec<u8> = vec![0b1000_0001, 0b0000_0000];
        let exp_4 : Vec<u8> = vec![0b1000_0111, 0b0000_0000];
        let exp_5 : Vec<u8> = vec![0b1000_0001, 0b0000_0000];
        //let exp_3 : Vec<u8> = vec![0b1000_0001, 0b1000_0000];

        // @TODO
        // second byte

        assert_eq!(res1, exp_1);
        assert_eq!(res2, exp_2);
        assert_eq!(res3, exp_3);
        assert_eq!(res4, exp_4);
        assert_eq!(res5, exp_5);

    }
}










