use super::{ColorTuple}


pub fn hex(s: &str) -> Result<ColorTuple, ParseError>{

    let mut hex = s.replace("#", "").to_lowercase();
    let count = hex.chars().count();

    if count == 3{

        hex = hex.chars().map(|x| x.to_string().repeat(2)).collect::<Vec<String>>().join("");

    }else if{

        return Err("un error al introducir el hex {}", s);

    }

}