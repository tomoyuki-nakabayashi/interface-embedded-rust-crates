#[cfg(test)]
mod tests {
    use bit_field::BitField;

    #[test]
    fn bit_manipulation() {
        let mut byte: u8 = 0b1010_0101;
        // 最下位ビット (インデックス0のビット) を取得する
        assert!(byte.get_bit(0));
        assert!(!byte.get_bit(1));

        byte.set_bit(1, true);
        assert!(byte.get_bit(1));

        assert_eq!(byte, 0b1010_0111);
    }

    #[test]
    fn bits_manipulation() {
        let mut byte: u8 = 0b1010_0101;
        assert_eq!(byte.get_bits(0..4), 0b0101);
        assert_eq!(byte.get_bits(4..8), 0b1010);

        byte.set_bits(0..4, 0b1111);
        assert_eq!(byte, 0b1010_1111);
    }

    #[test]
    fn range_operations() {
        let byte: u8 = 0b1010_0101;
        assert_eq!(byte.get_bits(..), byte);
        assert_eq!(byte.get_bits(..4), 0b0101);
        assert_eq!(byte.get_bits(4..), 0b1010);
        assert_eq!(byte.get_bits(1..=3), 0b010);
    }

    use bit_field::BitArray;

    #[test]
    fn bit_array() {
        let mut array = [0u8; 2];
        assert_eq!(array.bit_length(), 16);
        array.set_bit(1, true);
        array.set_bit(13, true);
        assert!(array.get_bit(1));
        assert!(array.get_bit(13));
        assert_eq!(array[0], 0b0000_0010);
        assert_eq!(array[1], 0b0010_0000);

        array.set_bits(6..10, 0b1111);
        assert_eq!(array[0], 0b1100_0010);
        assert_eq!(array[1], 0b0010_0011);
    }

    fn slice_manipulation(slice: &[u8]) {
        assert_eq!(slice.bit_length(), 16);
    }

    #[test]
    fn test_slice() {
        let array = [0u8; 2];
        slice_manipulation(&array);
    }
}
