#[cfg(test)]
mod tests {
    use crate::lzss::{LZSS, Pair};
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


        let res1 = LZSS::max_sub(&t1, &t2);
        let res2 = LZSS::max_sub(&t3, &t4);
        let res3 = LZSS::max_sub(&t5, &t6);
        let res4 = LZSS::max_sub(&t7, &t8);
        let res5 = LZSS::max_sub(&t9, &t10);
        let res6 = LZSS::max_sub(&t11, &t12);

        assert_eq!(res1, Some(Pair::new(2, 3)));
        assert_eq!(res2, Some(Pair::new(0, 6)));
        assert_eq!(res3, Some(Pair::new(2, 6)));
        assert_eq!(res4, Some(Pair::new(0, 4)));
        assert_eq!(res5, Some(Pair::new(0, 5)));
        assert_eq!(res6, Some(Pair::new(0, 7)));
    }
}
