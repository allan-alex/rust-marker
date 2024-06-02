use regex::Regex;

trait ValidatedEmail {
    fn get_inner(&self) -> &str;
}
trait ErroredEmail {
    fn get_inner(&self) -> &str;
}
struct Email(String);

#[derive(Debug)]
struct ValidEmail(String);

#[derive(Debug)]
struct ErrorEmail(String);
impl ValidatedEmail for ValidEmail {
    fn get_inner(&self) -> &str {
        &self.0
    }
}
impl ErroredEmail for ErrorEmail {
    fn get_inner(&self) -> &str {
        &self.0
    }
}

impl Email {
    fn validate_email(value: String) -> Result<ValidEmail, ErrorEmail> {
        let email_pattern = r"(?i)^[a-z0-9.+-]+@[a-z0-9.-]+\.[a-z]{2,}$";
        let re = Regex::new(email_pattern).unwrap();
        if re.is_match(&value) {
            Ok(ValidEmail(value))
        } else {
            Err(ErrorEmail(value))
        }
    }
}

fn process_email_message<T, U>(valid_email: Option<T>, error_email: Option<U>)
where
    T: ValidatedEmail + std::fmt::Debug,
    U: ErroredEmail + std::fmt::Debug
{
    if let Some(valid_email) = valid_email{
        println!("{} is a valid email", valid_email.get_inner());
    } else if let Some(error_email) = error_email {
        println!("{} is not a valid Email", error_email.get_inner());
    }
}


fn main() {
    let email = Email(String::from("alan@mail.com"));
    match Email::validate_email(email.0) {
        Ok(v_email) => process_email_message(Some(v_email), None::<ErrorEmail>),
        Err(err_email) => process_email_message(None::<ValidEmail>, Some(err_email))
    }
}