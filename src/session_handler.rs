use actix_web::{Error, HttpRequest, HttpMessage};
use actix_session::Session;

pub async fn index(session: Session, req: HttpRequest) -> Result<&'static str, Error> {
    
    let cookies_accepted = *req.extensions().get::<bool>().unwrap_or(&false);

    if cookies_accepted {
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