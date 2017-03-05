/// Calculates the last day of the month.
/// Leap years are not yet taken into account.
pub fn last_day_of_month(month: u32) -> u32 {
    let remainder = month % 2;

    if month == 2 {
        return 28;
    }
    if month <= 7 && remainder == 1 {
        return 31;
    }
    if month <= 7 && remainder == 0 {
        return 30;
    }
    if month > 7 && remainder == 0 {
        return 31;
    } else {
        return 30;
    }
}
