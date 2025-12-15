use regex::Regex;

pub fn is_valid_email(email: &str) -> bool {
    let email = email.trim_matches('"');
    let email_regex = Regex::new(
        r#"^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,6}))$"#
    ).unwrap();

    email_regex.is_match(email)
}

pub fn is_valid_password(password: &str) -> bool {
    let password = password.trim_matches('"');
    let length_ok = Regex::new(r"^.{6,18}$").unwrap().is_match(password);
    let has_digit = Regex::new(r"\d").unwrap().is_match(password);
    let has_upper = Regex::new(r"[A-Z]").unwrap().is_match(password);
    let has_lower = Regex::new(r"[a-z]").unwrap().is_match(password);
    let has_special = Regex::new(r"[^A-Za-z0-9]").unwrap().is_match(password);
    length_ok && has_digit && has_upper && has_lower && has_special
}
