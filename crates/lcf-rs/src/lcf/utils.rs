use nom::{
    Parser,
    bytes::complete::{take, take_while_m_n},
    combinator::{recognize, verify},
    multi::{length_data, many0},
};

pub fn parse_chunks(input: &[u8]) -> nom::IResult<&[u8], Vec<(i128, &[u8])>> {
    many0((parse_signed, length_data(parse_signed.map(|x| x as usize)))).parse(input)
}

pub fn parse_signed(input: &[u8]) -> nom::IResult<&[u8], i128> {
    let (input, bytes) = recognize((
        take_while_m_n(0, 4, |x| x >= 0x80),
        verify(take(1usize), |x: &[u8]| x[0] < 0x80),
    ))
    .parse(input)?;

    let mut value = 0i128;
    for byte in bytes {
        value <<= 7;
        value |= i128::from(byte & 0x7F);
    }

    Ok((input, value))
}

#[test]
fn test_parse_chunks() {
    let input = &[0x01u8, 0x02u8, 0x00u8, 0x00u8, 0xffu8] as &[u8];

    let (input, vec) = parse_chunks(input).unwrap();

    assert_eq!(input, &[0xffu8]);
    assert_eq!(vec, vec![(1, &[0x00u8, 0x00u8] as &[u8])]);
}

#[test]
fn test_parse_signed() {
    let input = &[0x93u8, 0x45u8] as &[u8];
    let (input, num) = parse_signed(input).unwrap();

    assert_eq!(input, &[]);
    assert_eq!(num, 2501);
}
