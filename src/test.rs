#[cfg(test)]
mod tests {
    use crate::lzss::LZSS;
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
}
