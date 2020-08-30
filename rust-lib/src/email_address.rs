extern crate pest;
extern crate pest_derive;
use pest::Parser;
use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Parser)]
#[grammar = "rfc5322.pest"]
struct RFC5322;

/// Email address struct.
///
/// # Examples
/// ```
/// use email_address_parser::EmailAddress;
///
/// assert!(EmailAddress::parse("foo@-bar.com", Some(true)).is_none());
/// let email = EmailAddress::parse("foo@bar.com", Some(true));
/// assert!(email.is_some());
/// let email = email.unwrap();
/// assert_eq!(email.get_local_part(), "foo");
/// assert_eq!(email.get_domain(), "bar.com");
/// assert_eq!(format!("{}", email), "foo@bar.com");
/// ```
#[wasm_bindgen]
#[derive(Debug)]
pub struct EmailAddress {
  local_part: String,
  domain: String,
}

#[wasm_bindgen]
impl EmailAddress {
  #![warn(missing_docs)]
  #![warn(missing_doc_code_examples)]
  // TODO: validate local part and domain

  /// Instantiates a new `EmailAddress`.
  /// 
  /// Accessible from WASM.
  /// 
  /// #Examples
  /// ```
  /// use email_address_parser::EmailAddress;
  /// 
  /// let email = EmailAddress::new("foo", "bar.com");
  /// ```
  pub fn new(local_part: &str, domain: &str) -> EmailAddress {
    EmailAddress {
      local_part: String::from(local_part),
      domain: String::from(domain),
    }
  }

  /// Parses a given string as an email address.
  /// 
  /// Accessible from WASM.
  /// 
  /// Returns `Some(EmailAddress)` if the parsing is successful, else `None`.
  /// #Examples
  /// ```
  /// use email_address_parser::EmailAddress;
  /// 
  /// // strict parsing
  /// let email = EmailAddress::parse("foo@bar.com", Some(true));
  /// assert!(email.is_some());
  /// let email = email.unwrap();
  /// assert_eq!(email.get_local_part(), "foo");
  /// assert_eq!(email.get_domain(), "bar.com");
  /// 
  /// // non-strict parsing
  /// let email = EmailAddress::parse("\u{0d}\u{0a} \u{0d}\u{0a} test@iana.org", None);
  /// assert!(email.is_some());
  /// 
  /// // parsing invalid address
  /// let email = EmailAddress::parse("test@-iana.org", Some(true));
  /// assert!(email.is_none());
  /// let email = EmailAddress::parse("test@-iana.org", None);
  /// assert!(email.is_none());
  /// let email = EmailAddress::parse("test", Some(true));
  /// assert!(email.is_none());
  /// let email = EmailAddress::parse("test", None);
  /// assert!(email.is_none());
  /// ```
  pub fn parse(input: &str, is_strict: Option<bool>) -> Option<EmailAddress> {
    let instantiate = |mut parsed: pest::iterators::Pairs<Rule>| {
      let mut parsed = parsed
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .into_inner();
      Some(EmailAddress {
        local_part: String::from(parsed.next().unwrap().as_str()),
        domain: String::from(parsed.next().unwrap().as_str()),
      })
    };
    let is_strict = is_strict.unwrap_or_default();
    match RFC5322::parse(Rule::address_single, input) {
      Ok(parsed) => instantiate(parsed),
      Err(_) => {
        if is_strict {
          None
        } else {
          match RFC5322::parse(Rule::address_single_obs, input) {
            Ok(parsed) => instantiate(parsed),
            Err(_) => None,
          }
        }
      }
    }
  }
  /// Returns the local part of the email address.
  /// 
  /// Note that if you are using this library from rust, then consider using the `get_local_part` method instead.
  /// This returns a cloned copy of the local part string, instead of a borrowed `&str`, and exists purely for WASM interoperability.
  /// 
  /// #Examples
  /// ```
  /// use email_address_parser::EmailAddress;
  /// 
  /// let email = EmailAddress::new("foo", "bar.com");
  /// assert_eq!(email.local_part(), "foo");
  /// 
  /// let email = EmailAddress::parse("foo@bar.com", Some(true)).unwrap();
  /// assert_eq!(email.local_part(), "foo");
  /// ```
  #[doc(hidden)]
  pub fn local_part(&self) -> String {
    self.local_part.clone()
  }
  /// Returns the domain of the email address.
  /// 
  /// Note that if you are using this library from rust, then consider using the `get_domain` method instead.
  /// This returns a cloned copy of the domain string, instead of a borrowed `&str`, and exists purely for WASM interoperability.
  /// 
  /// #Examples
  /// ```
  /// use email_address_parser::EmailAddress;
  /// 
  /// let email = EmailAddress::new("foo", "bar.com");
  /// assert_eq!(email.domain(), "bar.com");
  /// 
  /// let email = EmailAddress::parse("foo@bar.com", Some(true)).unwrap();
  /// assert_eq!(email.domain(), "bar.com");
  /// ```
  #[doc(hidden)]
  pub fn domain(&self) -> String {
    self.domain.clone()
  }
}

impl EmailAddress {
  #![warn(missing_docs)]
  #![warn(missing_doc_code_examples)]
  
  /// Returns the local part of the email address.
  /// 
  /// Not accessible from WASM.
  /// 
  /// #Examples
  /// ```
  /// use email_address_parser::EmailAddress;
  /// 
  /// let email = EmailAddress::new("foo", "bar.com");
  /// assert_eq!(email.get_local_part(), "foo");
  /// 
  /// let email = EmailAddress::parse("foo@bar.com", Some(true)).unwrap();
  /// assert_eq!(email.get_local_part(), "foo");
  /// ```
  pub fn get_local_part(&self) -> &str {
    self.local_part.as_str()
  }
  
  /// Returns the domain of the email address.
  /// 
  /// Not accessible from WASM.
  /// 
  /// #Examples
  /// ```
  /// use email_address_parser::EmailAddress;
  /// 
  /// let email = EmailAddress::new("foo", "bar.com");
  /// assert_eq!(email.get_domain(), "bar.com");
  /// 
  /// let email = EmailAddress::parse("foo@bar.com", Some(true)).unwrap();
  /// assert_eq!(email.get_domain(), "bar.com");
  /// ```
  pub fn get_domain(&self) -> &str {
    self.domain.as_str()
  }
}

impl fmt::Display for EmailAddress {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    formatter.write_fmt(format_args!("{}@{}", self.local_part, self.domain))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn email_address_instantiation_works() {
    let address = EmailAddress::new("foo", "bar.com");
    assert_eq!(address.get_local_part(), "foo");
    assert_eq!(address.get_domain(), "bar.com");
    assert_eq!(format!("{}", address), "foo@bar.com");
  }

  #[test]
  fn domain_rule_does_not_parse_dash_google_dot_com() {
    let address = RFC5322::parse(Rule::domain_complete, "-google.com");
    println!("{:#?}", address);
    assert_eq!(address.is_err(), true);
  }

  #[test]
  fn domain_rule_does_not_parse_dash_google_dot_com_obs() {
    let address = RFC5322::parse(Rule::domain_obs, "-google.com");
    println!("{:#?}", address);
    assert_eq!(address.is_err(), true);
  }

  #[test]
  fn domain_rule_does_not_parse_dash_google_dash_dot_com() {
    let address = RFC5322::parse(Rule::domain_complete, "-google-.com");
    println!("{:#?}", address);
    assert_eq!(address.is_err(), true);
  }

  #[test]
  fn domain_rule_parses_google_dash_dot_com() {
    let address = RFC5322::parse(Rule::domain_complete, "google-.com");
    println!("{:#?}", address);
    assert_eq!(address.is_err(), true);
  }

  #[test]
  fn domain_complete_punycode_domain() {
    let actual = RFC5322::parse(Rule::domain_complete, "xn--masekowski-d0b.pl");
    println!("{:#?}", actual);
    assert_eq!(actual.is_err(), false);
  }

  #[test]
  fn can_parse_deprecated_local_part() {
    let actual = RFC5322::parse(Rule::local_part_obs, "\"test\".\"test\"");
    println!("{:#?}", actual);
    assert_eq!(actual.is_err(), false);
  }

  #[test]
  fn can_parse_email_with_deprecated_local_part() {
    let actual = RFC5322::parse(Rule::address_single_obs, "\"test\".\"test\"@iana.org");
    println!("{:#?}", actual);
    assert_eq!(actual.is_err(), false);
  }

  #[test]
  fn can_parse_domain_with_space() {
    println!("{:#?}", RFC5322::parse(Rule::domain_obs, " iana .com"));
    let actual = EmailAddress::parse("test@ iana .com", None);
    println!("{:#?}", actual);
    assert_eq!(actual.is_some(), true, "test@ iana .com");
  }

  #[test]
  fn can_parse_email_with_cfws_near_at() {
    let email = " test @iana.org";
    let actual = EmailAddress::parse(&email, None);
    println!("{:#?}", actual);
    assert_eq!(format!("{}", actual.unwrap()), email);
  }

  #[test]
  fn can_parse_email_with_crlf() {
    let email = "\u{0d}\u{0a} test@iana.org";
    let actual = EmailAddress::parse(&email, None);
    println!("{:#?}", actual);
    assert_eq!(format!("{}", actual.unwrap()), email);
  }

  #[test]
  fn can_parse_local_part_with_space() {
    let actual = RFC5322::parse(Rule::address_single_obs, "test . test@iana.org");
    println!("{:#?}", actual);
    assert_eq!(actual.is_err(), false);
  }

  #[test]
  fn can_parse_domain_with_bel() {
    let actual = RFC5322::parse(Rule::domain_literal, "[RFC-5322-\u{07}-domain-literal]");
    println!("{:#?}", actual);
    assert_eq!(actual.is_err(), false);
  }
}
