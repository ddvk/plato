mod kobo;
mod fake;
mod remarkable;

use failure::Error;

pub use self::kobo::KoboBattery;
pub use self::fake::FakeBattery;
pub use self::remarkable::RemarkableBattery;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Status {
    Discharging,
    Charging,
    Charged,
    // Full,
    // Unknown
}

pub trait Battery {
    fn capacity(&mut self) -> Result<f32, Error>;
    fn status(&mut self) -> Result<Status, Error>;
}
