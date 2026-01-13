mod timer;

use std::time::Duration;

use timer::Timer;
use timer::TimerCommand;
use timer::TimerState;

#[tokio::main]
async fn main() {
    println!("timer-cli start");

    let work_duration = Duration::from_secs(10);

    let mut timer = Timer::new(work_duration);

    println!("메뉴 : 1)▶️  2)🧭");
    /*
       if 1)
           run_timer
        else if 2)
           run_setting
    */
    run_timer(&mut timer).await;
}

async fn run_timer(timer: &mut Timer) {
    /* 
        

        while let Some 
            timer.start()
     */

    timer.start();
    println!("{}", timer);

    loop {
        tokio::select! {

            _ = tokio::time::sleep(Duration::from_secs(1)), if timer.state == TimerState::Working => {
                println!("{}", timer);

                timer.update();

                if timer.state == TimerState::Inactive {
                    break
                }

            },

        }
    }
}

// setting
async fn run_setting() {}
