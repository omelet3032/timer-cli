#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Timer {
    pub work_duration: Duration,
    deadline: Instant,
    state: TimerState,
}

#[derive(Copy, Clone, Debug, PartialEq)]
    /*
       pre generated timer's duration change method
    */
    pub fn is_working(&self) -> bool {
        matches!(self.state, TimerState::Working)
    }

    pub fn is_inactive(&self) -> bool {
        matches!(self.state, TimerState::Inactive)
    }
    pub fn change_duration(&mut self, new_duration: Duration) {
        self.work_duration = new_duration
    }

    // pub fn initialize_duration(&mut self) {

