#[cfg(test)]
mod tests {
    use bitfield::{bitfield, BitRange, BitRangeMut};

    bitfield! {
        struct Byte(u8);
        impl Debug;
        flag1, set_flag1 : 0;
        flag2, set_flag2 : 1;
        field1, set_field1 : 7, 2;
    }

    #[test]
    fn test_bitfield() {
        let mut byte = Byte(0b1010_0101);
        assert_eq!(byte.flag1(), true);
        assert_eq!(byte.flag2(), false);
        assert_eq!(byte.field1(), 0b1010_01);

        byte.set_flag1(false);
        byte.set_flag2(true);
        byte.set_field1(0b1100_11);

        assert_eq!(byte.flag1(), false);
        assert_eq!(byte.flag2(), true);
        assert_eq!(byte.field1(), 0b1100_11);

        assert_eq!(byte.0, 0b1100_1110);
        println!("{:?}", byte);
    }

    bitfield! {
        struct AccessControl(u8);
        impl Debug;
        flag1, set_flag1 : 0;
        flag2, _ : 1;
        _, set_field1 : 7, 2;
    }

    #[test]
    fn test_access_control() {
        let mut ac = AccessControl(0b1010_0101);
        assert_eq!(ac.flag1(), true);
        assert_eq!(ac.flag2(), false);
        // assert_eq!(ac.field1(), 0b1010_01); compile error

        ac.set_flag1(false);
        // ac.set_flag2(true); // compile error
        ac.set_field1(0b1100_11);

        assert_eq!(ac.0, 0b1100_1100);
        println!("{:?}", ac);
    }

    bitfield! {
        struct Fields(u8);
        impl Debug;
        field1, set_field1 : 7, 0;
        field2, set_field2 : 5, 2;
    }

    #[test]
    fn test_fields() {
        let fields = Fields(0b1010_0101);
        assert_eq!(fields.field1(), 0b1010_0101);
        assert_eq!(fields.field2(), 0b1001);
        println!("{:?}", fields);
    }

    bitfield! {
        struct Bytes([u8]);
        impl Debug;
        u8;
        flag1, set_flag1 : 0;
        flag2, set_flag2 : 1;
        field1, set_field1 : 9, 2;
        flag3, set_flag3 : 14;
        flag4, set_flag4 : 15;
    }

    #[test]
    fn test_bytes() {
        let bytes = Bytes([0b0000_1111, 0b0101_0101]);
        assert_eq!(bytes.flag1(), true);
        assert_eq!(bytes.flag2(), true);
        assert_eq!(bytes.field1(), 0b01_0000_11);
        assert_eq!(bytes.flag3(), true);
        assert_eq!(bytes.flag4(), false);

        println!("{:?}", bytes);
    }

    bitfield! {
        struct MsbBytes(MSB0 [u8]);
        impl Debug;
        u8;
        flag1, set_flag1 : 0;
        flag2, set_flag2 : 1;
        field1, set_field1 : 9, 2;
        flag3, set_flag3 : 14;
        flag4, set_flag4 : 15;
    }

    #[test]
    fn test_msb_bytes() {
        let bytes = MsbBytes([0b0000_1111, 0b0101_0101]);
        assert_eq!(bytes.flag1(), false);
        assert_eq!(bytes.flag2(), false);
        assert_eq!(bytes.field1(), 0b00_1111_01);
        assert_eq!(bytes.flag3(), false);
        assert_eq!(bytes.flag4(), true);

        println!("{:?}", bytes);
    }

    #[test]
    fn test_bit_range() {
        let mut byte = Byte(0b1010_0101);
        assert_eq!(BitRange::<u8>::bit_range(&byte, 7, 2), 0b1010_01);
        byte.set_bit_range(7, 2, 0b1100_11);
        assert_eq!(BitRange::<u8>::bit_range(&byte, 7, 2), 0b1100_11);
        assert_eq!(byte.0, 0b1100_1101);
    }

    bitfield! {
        struct Integers([u8]);
        impl Debug;
        u8;
        u16, field1, set_field1 : 15, 2;
        field2, set_field2 : 14, 10;
    }

    fn type_of<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
    }

    #[test]
    fn test_integers() {
        let integers = Integers([0b1010_0101, 0b1100_0011]);
        assert_eq!(integers.field1(), 0b1100_0011_1010_01);
        assert_eq!(integers.field2(), 0b1000_0);
        println!("field1: {}", type_of(integers.field1()));
        println!("field2: {}", type_of(integers.field2()));
        println!("{:?}", integers);
    }

    bitfield! {
        #[derive(Clone, Copy)]
        struct BitManipulation(u8);
        impl Debug;
        impl BitAnd;
        impl BitOr;
        impl BitXor;
        flag1, set_flag1 : 0;
        flag2, set_flag2 : 1;
        flag3, set_flag3 : 2;
        flag4, set_flag4 : 3;
        field1, set_field1 : 7, 4;
    }

    #[test]
    fn test_bit_manipulation() {
        let bm = BitManipulation(0b1010_0101);
        assert_eq!(bm.flag1(), true);
        assert_eq!(bm.flag2(), false);
        assert_eq!(bm.flag3(), true);
        assert_eq!(bm.flag4(), false);
        assert_eq!(bm.field1(), 0b1010);
        println!("{:?}", bm);

        let bm2 = BitManipulation(0b1111_0000);
        let bm3 = bm | bm2;
        assert_eq!(bm3.0, 0b1111_0101);
        let bm4 = bm & bm2;
        assert_eq!(bm4.0, 0b1010_0000);
    }

    #[derive(Debug, PartialEq)]
    enum MyEnum {
        A,
        B,
        C,
        D,
    }

    impl From<u8> for MyEnum {
        fn from(value: u8) -> Self {
            match value {
                0 => MyEnum::A,
                1 => MyEnum::B,
                2 => MyEnum::C,
                3 => MyEnum::D,
                _ => unreachable!(),
            }
        }
    }

    impl From<MyEnum> for u8 {
        fn from(value: MyEnum) -> Self {
            match value {
                MyEnum::A => 0,
                MyEnum::B => 1,
                MyEnum::C => 2,
                MyEnum::D => 3,
            }
        }
    }

    bitfield! {
        struct EnumField(u8);
        impl Debug;
        u8, from into MyEnum, enum_field, set_enum_field : 1, 0;
        field1, set_field1 : 7, 2;
    }

    #[test]
    fn test_enum() {
        let mut ef = EnumField(0b1010_0101);
        assert_eq!(ef.enum_field(), MyEnum::B);
        ef.set_enum_field(MyEnum::D);
        assert_eq!(ef.0, 0b1010_0111);
    }
}
