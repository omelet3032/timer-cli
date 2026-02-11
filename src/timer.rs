use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Timer {
    pub work_duration: Duration,
    pub deadline: Instant,
    pub state: TimerState,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TimerState {
    Working,
    Inactive,
    Paused(Duration),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TimerCommand {
    Start, // 맨 처음 시작시 loop진입전 시작 명령
    Pause,
    Reset,
}

impl FromStr for TimerCommand {
    type Err =();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       match s {
            "start" => Ok(TimerCommand::Start), 
            "pause" => Ok(TimerCommand::Pause),
            "reset" => Ok(TimerCommand::Reset),
            _ => Err(()),
       } 
    }
}

impl Timer {
    pub fn new(work_duration: Duration) -> Self {
        Self {
            work_duration: work_duration,
            deadline: Instant::now(),
            state: TimerState::Inactive,
        }
    }
    /*
       pre generated timer's duration change method
    */
    pub fn change_duration(&mut self, new_duration: Duration) {
        self.work_duration = new_duration
    }

    // pub fn initialize_duration(&mut self) {
    //     self.work_duration

    // }
    pub fn start(&mut self) {
        match self.state {
            TimerState::Inactive => {
                self.deadline = Instant::now() + self.work_duration;
                self.state = TimerState::Working;
            }
            TimerState::Paused(remaining) => {
                self.deadline = Instant::now() + remaining;
                self.state = TimerState::Working;
            }
            _ => {}
        }
    }

    pub fn pause(&mut self) {
        if let TimerState::Working = self.state {
            let remaining = self.time_left();
            self.state = TimerState::Paused(remaining)
        }
    }

    pub fn reset(&mut self) {
        self.state = TimerState::Inactive;
        self.deadline = Instant::now() + self.work_duration;
    }

    pub fn update(&mut self) {
        if let TimerState::Working = self.state {
            if self.time_left().is_zero() {
                self.state = TimerState::Inactive;
            }
        }
    }

    fn time_left(&self) -> Duration {
        self.deadline
            .checked_duration_since(Instant::now())
            .unwrap_or(Duration::ZERO)
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let time_left = match self.state {
            TimerState::Paused(remaining) => remaining,
            TimerState::Inactive => self.work_duration,
            _ => self.time_left(),
        };

        let secs = time_left.as_secs_f64().ceil() as u64;

        let minutes_left = secs / 60;
        let seconds_left = secs % 60;

        write!(f, "{:02}:{:02}", minutes_left, seconds_left)
    }
}
