use time::{self, Date, Duration};

#[derive(Debug)]
enum TrainingCreationState {
    WaitingForDate,
    WaitingForExercises,
    WaitingForDuration,
    WaitingForNotes,
    Completed,
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

    // pub fn week_day_russian(&self) -> &str { 
    //     match &self.date.weekday() {
    //         time::Weekday::Monday => "Понедельник",
    //         time::Weekday::Tuesday => "Вторник",
    //         time::Weekday::Wednesday => "Среда",
    //         time::Weekday::Thursday => "Четверг",
    //         time::Weekday::Friday => "Пятница",
    //         time::Weekday::Saturday => "Суббота",
    //         time::Weekday::Sunday => "Воскресенье",
    //     }
    // }
}
