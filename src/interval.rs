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
        "d" => return Ok(Interval::Daily),
        "w" => return Ok(Interval::Weekly),
        "b" => return Ok(Interval::Biweekly),
        "m" => return Ok(Interval::Monthly),
        _ => return Err(io::Error::new(io::ErrorKind::Other,
                "Invalid interval flag stored"))
    }
}
