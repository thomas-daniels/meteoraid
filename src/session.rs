use crate::areas::Area;
use crate::field::Field;
use crate::meteor::{Meteor, Shower};
use crate::timestamp::Timestamp;

pub enum Event {
    Clouds(u8),
    AreasCounted(Vec<(usize, Area)>),
    BreakStart,
    BreakEnd,
    NewPeriod,
    Meteor(Meteor),
    Field(Field),
}

pub struct TimestampedEvent(pub Timestamp, pub Event);

pub struct Period {
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub teff: f64,
    pub limiting_magnitude: f64,
    pub field: Field,
    pub cloud_factor: f64,
    pub showers: Vec<Shower>,
    pub meteors: Vec<Meteor>,
}

pub struct Session {
    pub periods: Vec<Period>,
}
