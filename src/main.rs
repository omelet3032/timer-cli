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

    
    run_timer(&mut timer).await;
}

// 이게 timer 앱이려나
async fn run_timer(timer: &mut Timer) {
    /* 
        async fn main()함수가 실행된 후
        사용자가 start를 입력하면
        aync fn run_timer가 실행되어야 함

        tokio_select! ??
     */

    loop {
        timer.start();

        tokio::select! {

            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                timer.update();
                println!("{:?}", timer.state);
                println!("{}", timer);
            }
        }
    }


}

// setting
async fn run_setting() {

}
