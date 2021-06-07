
#[test]
fn hex_from_str_test() {
    //unimplemented!();

    fn parse_hex(s: &str) -> Result<Rgb, ParseError>{

        Rgb::from_hex_str(s);

    }

    assert_eq!(parse_hex("#FFFFFF").unwrap().to_css_str(), "{255,255,255}");
    assert_eq!(parse_hex("#999999").unwrap().to_css_str(), "{153,153,153}");
    assert_eq!(parse_hex("#7F7F7F").unwrap().to_css_str(), "{127, 127, 127}");
    assert!(parse_hex("").is_err());
    assert!(parse_hex("8s7d7sd").is_err());

}