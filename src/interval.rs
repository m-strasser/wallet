use std::io;

#[derive(Debug, Clone)]
pub enum Interval {
    Daily,
    Weekly,
    Biweekly,
    Monthly
}

pub fn interval_from_string(s: String) -> Result<Interval, io::Error> {
    match s.as_ref() {
        "Daily" => return Ok(Interval::Daily),
        "Weekly" => return Ok(Interval::Weekly),
        "Biweekly" => return Ok(Interval::Biweekly),
        "Monthly" => return Ok(Interval::Monthly),
        _ => return Err(io::Error::new(io::ErrorKind::Other,
                "Invalid interval flag stored"))
    }
}
