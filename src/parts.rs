use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::{BeyBit, BeyRachet, BeyTop};

#[derive(Clone, Deserialize, Serialize)]
pub struct PartsList {
    pub tops: Vec<BeyTop>,
    pub rachets: Vec<BeyRachet>,
    pub bits: Vec<BeyBit>,
}

async fn load_parts() -> Result<PartsList, Box<dyn Error>> {
    Ok(PartsList {
        tops: vec![],
        rachets: vec![],
        bits: vec![],
    })
}
