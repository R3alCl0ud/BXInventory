use std::{error::Error, sync::{LazyLock, Mutex}};



slint::include_modules!();


use parts::PartsList;
mod parts;

pub static PARTS: LazyLock<Mutex<PartsList>> = LazyLock::new(||Mutex::new(PartsList {
    tops: vec![],
    rachets: vec![],
    bits: vec![],
}));


#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>>{


    let app = App::new()?;
    app.run()?;
    Ok(())
}
