use time::{self, Date, Duration, OffsetDateTime};

#[derive(Debug)]
enum TrainingCreationState {
    WaitingForDate,
    WaitingForExercises,
    WaitingForDuration,
    WaitingForNotes,
    Completed,
}

#[derive(Debug)]
pub struct TrainingSession {
    date: Date,
    exercises: Vec<String>,
    duration: Option<Duration>,
    notes: String,
}

#[derive(Debug)]
pub struct TrainingSessionBuilder {
    date: Option<Date>,
    exercises: Vec<String>,
    duration: Option<Duration>,
    notes: Option<String>,
    state: TrainingCreationState,
}

impl TrainingSessionBuilder {
    pub fn new() -> Self {
        Self {
            date: None,
            exercises: vec![],
            duration: None,
            notes: None,
            state: TrainingCreationState::WaitingForDate,
        }
    }

    pub fn set_date(&mut self) {
        self.date = Some(OffsetDateTime::now_utc().date());
        self.state = TrainingCreationState::WaitingForExercises;
    }

    pub fn set_notes(&mut self, notes: String) {
        self.notes = Some(notes);
    }

    pub fn week_day_russian(&self) -> Result<&str, String> {
        match &self.date {
            Some(date) => match date.weekday() {
                time::Weekday::Monday => Ok("Понедельник"),
                time::Weekday::Tuesday => Ok("Вторник"),
                time::Weekday::Wednesday => Ok("Среда"),
                time::Weekday::Thursday => Ok("Четверг"),
                time::Weekday::Friday => Ok("Пятница"),
                time::Weekday::Saturday => Ok("Суббота"),
                time::Weekday::Sunday => Ok("Воскресенье"),
            },
            None => Err("Date is not set".to_string()),
        }
    }

    pub fn build(self) -> Option<TrainingSession> {
        Some(TrainingSession {
            date: self.date?,
            exercises: self.exercises,
            duration: self.duration,
            notes: self.notes?,
        })
    }
}
