use nom::{
  branch::alt,
  character::complete::{char, crlf, space1, tab},
  combinator::opt,
  multi::{many0, many1},
  sequence::pair,
  IResult,
};

// WSP = { " " | "\t" }
fn wsp(input: &str) -> IResult<&str, &str> {
  match alt((char(' '), tab))(input) {
    Ok((i, o)) => Ok((i, &String::from(o))),
    Err(e) => Err(e),
  }
}

// // FWS = _{ ((WSP* ~ CRLF)? ~ WSP+) | obs_FWS }
// fn fws(input: &str) -> IResult<&str, &str> {
//   pair(opt(pair(many0(wsp), crlf)), many1(wsp))(input)
// }

#[cfg(test)]
mod nom_tests {
  mod wsp_tests {
    use super::super::*;

    #[test]
    fn space_is_wsp() {
      assert_eq!(wsp(" "), Ok(("", " ")));
      assert_eq!(wsp(" ab"), Ok(("ab", " ")));
    }

    #[test]
    fn tab_is_wsp() {
      assert_eq!(wsp("\t"), Ok(("", "\t")));
      assert_eq!(wsp("\tab"), Ok(("ab", "\t")));
    }
  }
}
