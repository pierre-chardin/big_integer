// Integration test for `big_integer`.

use big_integer::big_integer::BigInteger;
use big_integer::big_integer::error::BigIntegerErrorKind;
use std::str::FromStr;

#[test]
fn to_string() {
    assert_eq!(BigInteger::zero().to_string(), "0");
    assert_eq!(BigInteger::one().to_string(), "1");

    {
        let value = u32::MAX - 43;
        let value_big_integer = BigInteger::from(value);
        assert_eq!(value_big_integer.to_string(), value.to_string());
    }

    {
        let value = i32::MAX - 4321;
        let value_big_integer = BigInteger::from(value);
        assert_eq!(value_big_integer.to_string(), value.to_string());
    }

    {
        let value = i32::MIN + 542;
        let value_big_integer = BigInteger::from(value);
        assert_eq!(value_big_integer.to_string(), value.to_string());
    }

    {
        let value = u64::MAX - 8121;
        let value_big_integer = BigInteger::from(value);
        assert_eq!(value_big_integer.to_string(), value.to_string());
    }

    {
        let value = i64::MAX - 319;
        let value_big_integer = BigInteger::from(value);
        assert_eq!(value_big_integer.to_string(), value.to_string());
    }

    {
        let value = i64::MIN + 1111;
        let value_big_integer = BigInteger::from(value);
        assert_eq!(value_big_integer.to_string(), value.to_string());
    }

    {
        let value = u128::MAX - 45432;
        let value_big_integer = BigInteger::from(value);
        assert_eq!(value_big_integer.to_string(), value.to_string());
    }

    {
        let value = i128::MAX - 5285;
        let value_big_integer = BigInteger::from(value);
        assert_eq!(value_big_integer.to_string(), value.to_string());
    }

    {
        let value = i128::MIN + 931;
        let value_big_integer = BigInteger::from(value);
        assert_eq!(value_big_integer.to_string(), value.to_string());
    }
}

#[test]
fn format_x() {
    let test_value: i64 = -66564364635241212;
    let value = BigInteger::from(test_value);

    assert_eq!(format!("{:o}", value), format!("-{:o}", -test_value));
}

#[test]
fn format() {
    for test_value in [
        i128::MAX,
        45,
        0,
        432,
        765432765434321,
        -25,
        -765,
        -66564364635241212,
    ] {
        let value = BigInteger::from(test_value);

        // Decimal output.
        assert_eq!(format!("{}", value), format!("{}", test_value));
        assert_eq!(format!("{:+}", value), format!("{:+}", test_value));
        assert_eq!(format!("{:#}", value), format!("{:#}", test_value));
        assert_eq!(format!("{:+#}", value), format!("{:+#}", test_value));

        assert_eq!(format!("{:?}", value), format!("{:?}", test_value));
        assert_eq!(format!("{:+?}", value), format!("{:+?}", test_value));
        assert_eq!(format!("{:#?}", value), format!("{:#?}", test_value));
        assert_eq!(format!("{:+#?}", value), format!("{:+#?}", test_value));

        assert_eq!(format!("{:04}", value), format!("{:04}", test_value));
        assert_eq!(format!("{:02}", value), format!("{:02}", test_value));
        assert_eq!(format!("{:050}", value), format!("{:050}", test_value));

        assert_eq!(format!("{:+04}", value), format!("{:+04}", test_value));
        assert_eq!(format!("{:+01}", value), format!("{:+01}", test_value));
        assert_eq!(format!("{:+050}", value), format!("{:+050}", test_value));

        if test_value >= 0 {
            // Lower hex output.
            assert_eq!(format!("{:x}", value), format!("{:x}", test_value));
            assert_eq!(format!("{:+x}", value), format!("{:+x}", test_value));
            assert_eq!(format!("{:#x}", value), format!("{:#x}", test_value));
            assert_eq!(format!("{:+#x}", value), format!("{:+#x}", test_value));

            assert_eq!(format!("{:04x}", value), format!("{:04x}", test_value));
            assert_eq!(format!("{:02x}", value), format!("{:02x}", test_value));
            assert_eq!(format!("{:050x}", value), format!("{:050x}", test_value));

            assert_eq!(format!("{:+04x}", value), format!("{:+04x}", test_value));
            assert_eq!(format!("{:+02x}", value), format!("{:+02x}", test_value));
            assert_eq!(format!("{:+050x}", value), format!("{:+050x}", test_value));

            // Upper hex output.
            assert_eq!(format!("{:X}", value), format!("{:X}", test_value));
            assert_eq!(format!("{:+X}", value), format!("{:+X}", test_value));
            assert_eq!(format!("{:#X}", value), format!("{:#X}", test_value));
            assert_eq!(format!("{:+#X}", value), format!("{:+#X}", test_value));

            assert_eq!(format!("{:04X}", value), format!("{:04X}", test_value));
            assert_eq!(format!("{:02X}", value), format!("{:02X}", test_value));
            assert_eq!(format!("{:050X}", value), format!("{:050X}", test_value));

            assert_eq!(format!("{:+04X}", value), format!("{:+04X}", test_value));
            assert_eq!(format!("{:+02X}", value), format!("{:+02X}", test_value));
            assert_eq!(format!("{:+050X}", value), format!("{:+050X}", test_value));

            // Octal output.
            assert_eq!(format!("{:o}", value), format!("{:o}", test_value));
            assert_eq!(format!("{:+o}", value), format!("{:+o}", test_value));
            assert_eq!(format!("{:#o}", value), format!("{:#o}", test_value));
            assert_eq!(format!("{:+#o}", value), format!("{:+#o}", test_value));

            assert_eq!(format!("{:04o}", value), format!("{:04o}", test_value));
            assert_eq!(format!("{:02o}", value), format!("{:02o}", test_value));
            assert_eq!(format!("{:050o}", value), format!("{:050o}", test_value));

            assert_eq!(format!("{:+04o}", value), format!("{:+04o}", test_value));
            assert_eq!(format!("{:+02o}", value), format!("{:+02o}", test_value));
            assert_eq!(format!("{:+050o}", value), format!("{:+050o}", test_value));

            // Binary output.
            assert_eq!(format!("{:b}", value), format!("{:b}", test_value));
            assert_eq!(format!("{:+b}", value), format!("{:+b}", test_value));
            assert_eq!(format!("{:#b}", value), format!("{:#b}", test_value));
            assert_eq!(format!("{:+#b}", value), format!("{:+#b}", test_value));

            assert_eq!(format!("{:04b}", value), format!("{:04b}", test_value));
            assert_eq!(format!("{:02b}", value), format!("{:02b}", test_value));
            assert_eq!(format!("{:050b}", value), format!("{:050b}", test_value));

            assert_eq!(format!("{:+04b}", value), format!("{:+04b}", test_value));
            assert_eq!(format!("{:+02b}", value), format!("{:+02b}", test_value));
            assert_eq!(format!("{:+050b}", value), format!("{:+050b}", test_value));
        } else {
            assert_eq!(format!("{:x}", value), format!("-{:x}", -test_value));
            assert_eq!(format!("{:+x}", value), format!("-{:x}", -test_value));
            assert_eq!(format!("{:#x}", value), format!("-{:#x}", -test_value));
            assert_eq!(format!("{:+#x}", value), format!("-{:#x}", -test_value));

            assert_eq!(format!("{:04x}", value), format!("-{:03x}", -test_value));
            assert_eq!(format!("{:02x}", value), format!("-{:01x}", -test_value));
            assert_eq!(format!("{:050x}", value), format!("-{:049x}", -test_value));

            assert_eq!(format!("{:+04x}", value), format!("-{:03x}", -test_value));
            assert_eq!(format!("{:+02x}", value), format!("-{:01x}", -test_value));
            assert_eq!(format!("{:+050x}", value), format!("-{:049x}", -test_value));

            // Upper hex output.
            assert_eq!(format!("{:X}", value), format!("-{:X}", -test_value));
            assert_eq!(format!("{:+X}", value), format!("-{:X}", -test_value));
            assert_eq!(format!("{:#X}", value), format!("-{:#X}", -test_value));
            assert_eq!(format!("{:+#X}", value), format!("-{:#X}", -test_value));

            assert_eq!(format!("{:04X}", value), format!("-{:03X}", -test_value));
            assert_eq!(format!("{:02X}", value), format!("-{:01X}", -test_value));
            assert_eq!(format!("{:050X}", value), format!("-{:049X}", -test_value));

            assert_eq!(format!("{:+04X}", value), format!("-{:03X}", -test_value));
            assert_eq!(format!("{:+02X}", value), format!("-{:01X}", -test_value));
            assert_eq!(format!("{:+050X}", value), format!("-{:049X}", -test_value));

            // Octal output.
            assert_eq!(format!("{:o}", value), format!("-{:o}", -test_value));
            assert_eq!(format!("{:+o}", value), format!("-{:o}", -test_value));
            assert_eq!(format!("{:#o}", value), format!("-{:#o}", -test_value));
            assert_eq!(format!("{:+#o}", value), format!("-{:#o}", -test_value));

            assert_eq!(format!("{:04o}", value), format!("-{:03o}", -test_value));
            assert_eq!(format!("{:02o}", value), format!("-{:01o}", -test_value));
            assert_eq!(format!("{:050o}", value), format!("-{:049o}", -test_value));

            assert_eq!(format!("{:+04o}", value), format!("-{:03o}", -test_value));
            assert_eq!(format!("{:+02o}", value), format!("-{:01o}", -test_value));
            assert_eq!(format!("{:+050o}", value), format!("-{:049o}", -test_value));

            // Binary output.
            assert_eq!(format!("{:b}", value), format!("-{:b}", -test_value));
            assert_eq!(format!("{:+b}", value), format!("-{:b}", -test_value));
            assert_eq!(format!("{:#b}", value), format!("-{:#b}", -test_value));
            assert_eq!(format!("{:+#b}", value), format!("-{:#b}", -test_value));

            assert_eq!(format!("{:04b}", value), format!("-{:03b}", -test_value));
            assert_eq!(format!("{:02b}", value), format!("-{:01b}", -test_value));
            assert_eq!(format!("{:050b}", value), format!("-{:049b}", -test_value));

            assert_eq!(format!("{:+04b}", value), format!("-{:03b}", -test_value));
            assert_eq!(format!("{:+02b}", value), format!("-{:01b}", -test_value));
            assert_eq!(format!("{:+050b}", value), format!("-{:049b}", -test_value));
        }
    }
}

#[test]
fn from_str() {
    {
        let value_string = "5432122132331313131313131313131313131313";
        let value_big_integer = BigInteger::from_str(value_string).unwrap();
        assert_eq!(value_big_integer.to_string(), value_string);
    }

    {
        let value_string = "+6323232323232323231231777553532";
        let value_big_integer = BigInteger::from_str(value_string).unwrap();
        assert_eq!(value_big_integer.to_string(), value_string[1..]);
    }

    {
        let value_string = "-3232332323";
        let value_big_integer = BigInteger::from_str(value_string).unwrap();
        assert_eq!(value_big_integer.to_string(), value_string);
    }

    {
        let value_string = "-323XX2332323";
        let err = BigInteger::from_str(value_string).unwrap_err();
        assert_eq!(err.kind(), &BigIntegerErrorKind::InvalidDigit);
    }
}

#[test]
fn from_str_radix() {
    // (radix, string_value, to_string())
    let test_data: [(u32, &str, fn(v: &BigInteger) -> String); 3] = [
        (
            16,
            "FCFF4FAF3FFF1FFF8FFF4FFFF002EF5FCeFFaFF3FFFF6FFF7FF8FFbFFF9F8",
            |v: &BigInteger| format!("{:X}", v),
        ),
        (
            8,
            "512376543222121211313362164323122330011331131313313124540",
            |v: &BigInteger| format!("{:o}", v),
        ),
        (
            2,
            "10111100001110011111110000111110000111111100000111100111010101010000011",
            |v: &BigInteger| format!("{:b}", v),
        ),
    ];

    // (radix, string_value)
    let err_test_data = [
        (
            16,
            "FCFF4FAF3FFF1FFF8FFF4FFFF002EF5FCeFFGaFF3FFFF6FFF7FF8FFbFFF9F8",
        ),
        (
            8,
            "5123765432221212113133621643231223300811331131313313124540",
        ),
        (
            2,
            "101111000011100111111100001111100001111111000200111100111010101010000011",
        ),
    ];

    // Positive numbers.
    for (radix, value_string, to_str) in test_data {
        let value_big_integer = BigInteger::from_str_radix(value_string, radix).unwrap();
        assert_eq!(to_str(&value_big_integer), value_string.to_uppercase());

        let negative_value_big_integer;
        {
            let negative_value_string = String::from_str("-").unwrap() + value_string;
            negative_value_big_integer =
                BigInteger::from_str_radix(&negative_value_string, radix).unwrap();
            assert_eq!(
                to_str(&negative_value_big_integer),
                negative_value_string.to_uppercase()
            );
        }

        {
            let value_string_with_leading_zeros = String::from_str("000").unwrap() + value_string;
            let value_big_integer_other =
                BigInteger::from_str_radix(&value_string_with_leading_zeros, radix).unwrap();
            assert_eq!(value_big_integer_other, value_big_integer);
        }

        {
            let value_string_with_plus = String::from_str("+000").unwrap() + value_string;
            let value_big_integer_other =
                BigInteger::from_str_radix(&value_string_with_plus, radix).unwrap();
            assert_eq!(value_big_integer_other, value_big_integer);
        }

        {
            let negative_value_string_with_leading_zeros =
                String::from_str("-000").unwrap() + value_string;
            let negative_value_big_integer_other =
                BigInteger::from_str_radix(&negative_value_string_with_leading_zeros, radix)
                    .unwrap();
            assert_eq!(negative_value_big_integer_other, negative_value_big_integer);
        }
    }

    for (radix, value_string) in err_test_data {
        let err = BigInteger::from_str_radix(value_string, radix).unwrap_err();
        assert_eq!(err.kind(), &BigIntegerErrorKind::InvalidDigit);
    }
}

#[test]
fn from_bool() {
    let a = BigInteger::from(false);
    assert_eq!(a, BigInteger::zero());

    let b = BigInteger::from(true);
    assert_eq!(b, BigInteger::one())
}

#[test]
fn from_char() {
    assert_eq!(BigInteger::from('a'), BigInteger::from(u32::from('a')));
    assert_eq!(BigInteger::from('z'), BigInteger::from(u32::from('z')));
    assert_eq!(BigInteger::from('é'), BigInteger::from(u32::from('é')));
}

#[derive(Default)]
struct TestDefault {
    value1: BigInteger,
    value2: BigInteger,
}

#[test]
fn default() {
    let test = TestDefault::default();
    assert_eq!(test.value1.to_string(), "0");
    assert_eq!(test.value2.to_string(), "0");
}

#[test]
fn equal() {
    assert_eq!(BigInteger::zero(), BigInteger::zero());
    assert_eq!(BigInteger::one(), BigInteger::one());
    assert_ne!(BigInteger::zero(), BigInteger::one());
    assert_ne!(BigInteger::one(), BigInteger::zero());

    {
        let value1_string = "5341244814206912412774132148472101346641124414";
        let value1 = BigInteger::from_str(value1_string).unwrap();
        let value1_copy = BigInteger::from_str(value1_string).unwrap();
        assert_eq!(value1, value1_copy);
        assert_ne!(value1, BigInteger::zero());
        assert_ne!(value1, BigInteger::one());

        let value2 =
            BigInteger::from_str("5341244814206912412774132148472101346641124410").unwrap();
        assert_ne!(value1, value2);
    }

    {
        let value1_string = "-44814206912412774132148472101346641124414";
        let value1 = BigInteger::from_str(value1_string).unwrap();
        let value1_copy = BigInteger::from_str(value1_string).unwrap();
        assert_eq!(value1, value1_copy);
        assert_ne!(value1, BigInteger::zero());
        assert_ne!(value1, BigInteger::one());

        let value2 = BigInteger::from_str("23232").unwrap();
        assert_ne!(value1, value2);
    }
}

#[test]
fn clone() {
    assert_eq!(BigInteger::zero().clone(), BigInteger::zero());
    assert_eq!(BigInteger::one().clone(), BigInteger::one());

    let value1 = BigInteger::from_str("6646132452567387694328008845").unwrap();
    let value2 = value1.clone();
    assert_eq!(value1, value2);
}

#[test]
fn from_str_zero() {
    assert_eq!(BigInteger::zero(), BigInteger::from_str("0").unwrap());
}

#[test]
fn add2() {
    assert_eq!(BigInteger::zero() + BigInteger::zero(), BigInteger::zero());
    assert_eq!(BigInteger::zero() + BigInteger::one(), BigInteger::one());
    assert_eq!(BigInteger::one() + BigInteger::zero(), BigInteger::one());

    // Values of each tupple: a, b, a*b
    let test_data = [
        (
            "621345890643261965432186543113853297441904443",
            "8643218964031223565422000000",
            "621345890643261974075405507145076862863904443",
        ),
        (
            "-621345890643261965432186543113853297441904443",
            "-8643218964031223565422000000",
            "-621345890643261974075405507145076862863904443",
        ),
        (
            "21621345890643261443234232965432186543113853297441904443",
            "-21621345890643261443234232965432186543113853297441904443",
            "0",
        ),
    ];

    for data in &test_data {
        let a = BigInteger::from_str(data.0).unwrap();
        let b = BigInteger::from_str(data.1).unwrap();

        let s1 = BigInteger::from_str(data.2).unwrap();

        assert_eq!(&a + &b, s1);
        assert_eq!(&a + b.clone(), s1);
        assert_eq!(a.clone() + &b, s1);
        assert_eq!(a.clone() + b.clone(), s1);

        let c = &a + BigInteger::zero();
        assert_eq!(c, a);
        assert_eq!(BigInteger::zero() + &b, b);

        let mut d = a.clone();
        d += &b;
        assert_eq!(d, s1);

        d += &BigInteger::zero();
        assert_eq!(&d, &s1);
    }
}

#[test]
fn mul() {
    assert_eq!(BigInteger::zero() * BigInteger::zero(), BigInteger::zero());
    assert_eq!(BigInteger::one() * BigInteger::one(), BigInteger::one());

    // Values of each tupple: a, b, a*b
    let test_data = [
        (
            "94216502321054376443222300543223455",
            "7645388531954212345777324452245224",
            "720321766366226559177076379734170052180946354147405194899833088528920",
        ),
        (
            "-94216502321054376443222300543223455",
            "7645388531954212345777324452245224",
            "-720321766366226559177076379734170052180946354147405194899833088528920",
        ),
        (
            "-94216502321054376443222300543223455",
            "-7645388531954212345777324452245224",
            "720321766366226559177076379734170052180946354147405194899833088528920",
        ),
        (
            "1000000000000000000000000000000",
            "4444444444444445678432298542",
            "4444444444444445678432298542000000000000000000000000000000",
        ),
    ];

    for data in &test_data {
        let a = BigInteger::from_str(data.0).unwrap();
        let b = BigInteger::from_str(data.1).unwrap();

        let p1 = BigInteger::from_str(data.2).unwrap();

        assert_eq!(&a * &b, p1);
        assert_eq!(&a * b.clone(), p1);
        assert_eq!(a.clone() * &b, p1);
        assert_eq!(a.clone() * b.clone(), p1);

        let c = &a * BigInteger::zero();
        assert_eq!(c, BigInteger::zero());
        assert_eq!(BigInteger::zero() * &b, BigInteger::zero());

        assert_eq!(&a * BigInteger::one(), a);
        assert_eq!(BigInteger::one() * &b, b);

        let mut d = a.clone();
        d *= &b;
        assert_eq!(d, p1);

        d *= &BigInteger::one();
        assert_eq!(d, p1);

        d *= &BigInteger::zero();
        assert_eq!(&d, &BigInteger::zero());
    }
}
