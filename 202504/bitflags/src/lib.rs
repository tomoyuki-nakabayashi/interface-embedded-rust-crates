#[cfg(test)]
mod tests {
    use bitflags::{bitflags, parser};

    bitflags! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct Flags: u8 {
            const A = 0b00000001;
            const B = 0b00000010;
            const C = 0b00000100;
        }
    }

    #[test]
    fn test_logical_operations() {
        let ac = Flags::A | Flags::C;
        let bc = Flags::B | Flags::C;
        // OR
        assert_eq!((ac | bc), Flags::A | Flags::B | Flags::C);
        // AND
        assert_eq!((ac & bc), Flags::C);
        // NOT
        assert_eq!(!ac, Flags::B);
        // XOR
        assert_eq!((ac ^ bc), Flags::A | Flags::B);
        // (ac & !bc) == A
        assert_eq!((ac - bc), Flags::A);
    }

    #[test]
    fn test_basic_methods() {
        // empty()はフラグが1つも立っていない状態
        let none = Flags::empty();
        assert_eq!(none.is_empty(), true);
        // bits()はフラグのビット表現を返す
        assert_eq!(none.bits(), 0b00000000);

        // all()は定義した全てのフラグが立っている状態
        let all = Flags::all();
        assert_eq!(all.is_all(), true);
        assert_eq!(all.bits(), 0b00000111);

        let ab = Flags::A | Flags::B;
        assert_eq!(ab.bits(), 0b00000011);
        // from_bits_retain()は指定した整数値のビット表現からフラグ管理構造体の値を生成する
        assert_eq!(Flags::from_bits_retain(0b00000011), ab);
        // contains()は指定したフラグが立っているかどうかを返す
        assert!(ab.contains(Flags::A));

        // remove() / insert() はフラグの削除・追加を行う
        let mut flags = Flags::A | Flags::B;
        flags.remove(Flags::A);
        assert_eq!(flags.contains(Flags::A), false);
        assert_eq!(flags.contains(Flags::B), true);

        flags.insert(Flags::A);
        assert_eq!(flags.contains(Flags::A), true);
        assert_eq!(flags.contains(Flags::B), true);
    }

    #[test]
    fn test_iterator() {
        let flags = Flags::A | Flags::C;
        let mut iter = flags.iter();
        // 立っているビットフラグを順番に返す
        assert_eq!(iter.next(), Some(Flags::A));
        assert_eq!(iter.next(), Some(Flags::C));
        assert_eq!(iter.next(), None);

        // forループで立っているビットを順番に取得する
        for flag in flags {
            match flag {
                Flags::A => assert_eq!(flag, Flags::A),
                Flags::C => assert_eq!(flag, Flags::C),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_from_str() {
        assert_eq!(parser::from_str::<Flags>("A").unwrap(), Flags::A);
        assert_eq!(parser::from_str::<Flags>("A | B").unwrap(), Flags::A | Flags::B);
        assert_eq!(parser::from_str::<Flags>("0x01").unwrap(), Flags::A);
        assert_eq!(parser::from_str::<Flags>("0x01 | B").unwrap(), Flags::A | Flags::B);
    }

    #[test]
    fn test_to_str() {
        let flags = Flags::A | Flags::B;
        let mut s = String::new();
        parser::to_writer(&flags, &mut s).unwrap();
        assert_eq!(s, "A | B");
    }

    bitflags! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct MultiBitFlags: u8 {
            const MA = 0b00000011;
            const MB = 0b00001111;
        }
    }

    #[test]
    fn test_multi_bit_flags() {
        let ab = MultiBitFlags::from_bits_retain(0b00000111);
        assert_eq!(ab.contains(MultiBitFlags::MA), true);
        assert_eq!(ab.contains(MultiBitFlags::MB), false);
    }
}
