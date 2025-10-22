use time;

// training 
// -- start date/time 
// -- end date/time
// -- day
// -- time
// -- date
// -- exercises vec![]


#[derive(Debug)]
pub struct Training {
    date: time::Date
}

impl Training {
    pub fn new() -> Self {
        Self {
            date: time::OffsetDateTime::now_utc().date()
        }
    }

    pub fn week_day_russian(&self) -> &str {
        match &self.date.weekday() {
            time::Weekday::Monday => "Понедельник",
            time::Weekday::Tuesday => "Вторник",
            time::Weekday::Wednesday => "Среда",
            time::Weekday::Thursday => "Четверг",
            time::Weekday::Friday => "Пятница",
            time::Weekday::Saturday => "Суббота",
            time::Weekday::Sunday => "Воскресенье",
        }
    }
}
