use chrono::{Duration, Utc};

#[derive(Debug)]
pub struct ReviewData {
    pub interval: u32,
    pub repetitions: u32,
    pub ease_factor: f64,
}

pub fn calculate_next_review(data: &mut ReviewData, success: bool) -> chrono::DateTime<Utc> {
    if success {
        data.repetitions += 1;
        data.interval = if data.repetitions == 1 {
            1
        } else if data.repetitions == 2 {
            6
        } else {
            (data.interval as f64 * data.ease_factor) as u32
        };
    } else {
        data.repetitions = 0;
        data.interval = 1;
    }

    Utc::now() + Duration::days(data.interval.into())
}
