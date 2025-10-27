use actix_session::Session;
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse};

pub async fn index(session: Session, req: HttpRequest) -> Result<&'static str, Error> {
    let cookie_accepted = req
        .cookie("accepted")
        .map(|c| c.value() == "true")
        .unwrap_or(false); // consent cookie 

    if cookie_accepted {
        // access the session state
        if let Some(count) = session.get::<i32>("counter")? {
            println!("SESSION value: {}", count);
            // modify the session state
            session.insert("counter", count + 1)?;
        } else {
            session.insert("counter", 1)?;
        }
    } else {
        println!("User didn't accept cookies.")
    }

    Ok("Welcome!")
}
