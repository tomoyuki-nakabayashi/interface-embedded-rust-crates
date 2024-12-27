#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use deku::prelude::*;

    #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    struct SampleA {
        #[deku(bits = "1")]
        flag_a: bool,
        #[deku(bits = "3")]
        field_a: u8,
        #[deku(bits = "4")]
        field_b: u8,
        field_c: u16,
    }

    #[test]
    fn basic_serialize_deserialize() {
        let data: Vec<u8> = vec![0b1010_1101, 0xEF, 0xBE];
        let (_rest, val) = SampleA::from_bytes((data.as_ref(), 0)).unwrap();
        assert_eq!(
            val,
            SampleA {
                flag_a: true,
                field_a: 0b010,
                field_b: 0b1101,
                field_c: 0xBEEF
            }
        );

        let val = SampleA {
            flag_a: false,
            field_a: 0b101,
            field_b: 0b0101,
            field_c: 0xABCD,
        };
        let ret = val.to_bytes().unwrap();
        assert_eq!(ret, vec![0b0101_0101, 0xCD, 0xAB]);
    }

    #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    #[deku(endian = "big")] // フィールドのエンディアンをビッグエンディアンに設定
    struct SampleB {
        #[deku(bits = "1")]
        flag_a: bool,
        #[deku(bits = "3")]
        field_a: u8,
        #[deku(bits = "4")]
        field_b: u8,
        field_c: u16, // ビッグエンディアンでシリアライズ / デシリアライズされる
    }

    #[test]
    fn basic_serialize_deserialize_big_endian() {
        let data: Vec<u8> = vec![0b1010_1101, 0xCD, 0xAB];
        let (_rest, val) = SampleB::from_bytes((data.as_ref(), 0)).unwrap();
        assert_eq!(
            val,
            SampleB {
                flag_a: true,
                field_a: 0b101,
                field_b: 0b101,
                field_c: 0xCDAB
            }
        );
    }

    #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    struct Sub(u8);

    #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    struct Composed {
        sub: Sub,
        field_a: u8,
    }

    #[test]
    fn basic_serialize_deserialize_composed() {
        let data: Vec<u8> = vec![0x01, 0x02];
        let (_rest, val) = Composed::from_bytes((data.as_ref(), 0)).unwrap();
        assert_eq!(
            val,
            Composed {
                sub: Sub(0x01),
                field_a: 0x02
            }
        );

        let data_out = val.to_bytes().unwrap();
        assert_eq!(data_out, data);
    }

    #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    struct VariableLengthBytes {
        #[deku(update = "self.data.len()")]
        length: u8,
        #[deku(count = "size")]
        value: Vec<u8>,
    }

    #[test]
    fn basic_serialize_deserialize_variable_length_bytes() {
        // 先頭の`0x02`がデータの長さになる。今回は`Vec<u8>`なのでu8のデータが2個読み込まれる。
        // `0xCC, 0xDD`は余分なデータ
        let data: Vec<u8> = vec![0x02, 0xAA, 0xBB, 0xCC, 0xDD];
        let (rest, mut val) = VariableLengthBytes::from_bytes((data.as_ref(), 0)).unwrap();
        assert_eq!(
            val,
            VariableLengthBytes {
                length: 0x02,           // 長さ = 2
                value: vec![0xAA, 0xBB] // 2バイト分読み込まれる
            }
        );

        // 余分につけてあったデータは残ったデータとして返ってくる
        assert_eq!(rest.0, &[0xCCu8, 0xDD]);

        // 1バイト分データを追加して、シリアライズすうｒ
        val.data.push(0xEE);

        // この時点ではlengthフィールドが更新されていないので2バイトしかシリアライズされない
        let data_out = val.to_bytes().unwrap();
        assert_eq!(data_out, vec![0x02, 0xAA, 0xBB]);

        // update()を呼び出すとlengthフィールドが更新されて3バイトがシリアライズされる
        val.update().unwrap();
        let data_out = val.to_bytes().unwrap();
        assert_eq!(data_out, vec![0x03, 0xAA, 0xBB, 0xEE]);
    }

    #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    #[deku(id_type = "u8")]
    enum TLV {
        #[deku(id = "0x01")]
        A,
        #[deku(id = "0x02")]
        B(u16),
        #[deku(id = "0x03")]
        C(VariableLengthBytes),
    }

    #[test]
    fn basic_serialize_deserialize_enum() {
        let data: Vec<u8> = vec![0x01, 0x02, 0x34, 0x12, 0x03, 0x04, 0xAA, 0xBB, 0xCC, 0xDD];
        let mut cursor = Cursor::new(data);
        let (_, a) = TLV::from_reader((&mut cursor, 0)).unwrap();
        assert_eq!(
            a,
            TLV::A
        );

        let (_, b) = TLV::from_reader((&mut cursor, 0)).unwrap();
        assert_eq!(
            b,
            TLV::B(0x1234)
        );

        let (_, c) = TLV::from_reader((&mut cursor, 0)).unwrap();
        assert_eq!(
            c,
            TLV::C(VariableLengthBytes {
                count: 0x04,
                data: vec![0xAA, 0xBB, 0xCC, 0xDD]
            })
        );
    }
}
