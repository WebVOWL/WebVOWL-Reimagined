use actix_session::Session;
use actix_web::Error;

pub async fn index(session: Session) -> Result<&'static str, Error> {
    // access the session state
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        // modify the session state
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }

    Ok("Welcome!")
}