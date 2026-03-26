use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Timer {
    pub work_duration: Duration,
    deadline: Instant,
    state: TimerState,
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
    Quit,
}

impl FromStr for TimerCommand {
    type Err = std::io::Error;

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        match command {
            "start" | "s" => Ok(TimerCommand::Start),
            "pause" | "p" => Ok(TimerCommand::Pause),
            "reset" | "r" => Ok(TimerCommand::Reset),
            "quit" | "q" => Ok(TimerCommand::Quit),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("지원하지 않는 명령어 : {}", command))),
        }
    }
}

pub enum TimerDuration {
    A30,
    B60,
    C90,
}

impl FromStr for TimerDuration {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(TimerDuration::A30),
            "2" => Ok(TimerDuration::B60),
            "3" => Ok(TimerDuration::C90),
            _ => Err("Not Supported".to_string()),
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

    pub fn deadline(&self) -> Instant {
        self.deadline
    }

    pub fn deactivate(&mut self) {
        self.state = TimerState::Inactive;
    }

    pub fn is_working(&self) -> bool {
        matches!(self.state, TimerState::Working)
    }

    pub fn is_inactive(&self) -> bool {
        matches!(self.state, TimerState::Inactive)
    }

    pub fn change_duration(&mut self, new_duration: Duration) {
        self.work_duration = new_duration
    }

    pub fn start(&mut self) {
        let now = Instant::now();

        let duration = match self.state {
            TimerState::Inactive => self.work_duration,
            TimerState::Paused(remaining) => remaining,
            TimerState::Working => return,
        };

        self.deadline = now + duration;
        self.state = TimerState::Working;
    }

    pub fn pause(&mut self) {
        if let TimerState::Working = self.state {
            let remaining = self.time_left();
            self.state = TimerState::Paused(remaining)
        }
    }

    pub fn reset(&mut self) {
        self.state = TimerState::Inactive;
    }

    // for dioxus
    /*   pub fn update(&mut self) {
        if let TimerState::Working = self.state {
            if self.time_left().is_zero() {
                self.state = TimerState::Inactive;
            }
        }
    } */

    fn time_left(&self) -> Duration {
        self.deadline
            .checked_duration_since(Instant::now())
            .unwrap_or(Duration::ZERO)
    }
}

impl std::fmt::Display for Timer {
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
