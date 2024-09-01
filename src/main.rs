use dotenv;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::env;


fn main() {
    let arg: Vec<String> = std::env::args().collect();

    let dispatch_mail = arg[1].clone();

    let regex_email = regex::Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
    if !regex_email.is_match(&dispatch_mail) {
        println!("{} is not a valud email addr.", dispatch_mail);
        return;
    }
    if arg.len < 3 {
        println!("Usage: {} <email> <message>", arg[0]);
        return;
    }
    let message = arg[2..].join(" ");
    // Subject for the email
    let sub: String = arg[2..6].join(" ");

    // Here buildingg the actual regex_email
    let email = Message::builder()
        .from()
        .to(dispatch_mail.parse().unwrap())
        .subject(sub)
        .body(message)
        .unwrap();

    let cred = Credentials::new(.unwrap());

    let mail = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(cred)
        .build();

    match mail.send(&email){
        Ok(_) => println!("Success: Email Sent"),
        Err(err) => panic!("Couldn't send email: {} ", err),
    }
}
