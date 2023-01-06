use regex::Regex;

lazy_static! {
    /// Pattern for validating a password is 12+ characters and contains only the specified characters
    pub static ref PASSWORD_PATTERN: Regex =
        Regex::new(r#"[\w`~!@#$%^&*()-+={}\[\];:,.?]{12,}"#).expect("Invalid PASSWORD_PATTERN");
    pub static ref USERNAME_PATTERN: Regex =
        Regex::new(r#"[A-Za-z][A-Za-z0-9]*"#).expect("Invalid USERNAME_PATTERN");
}
