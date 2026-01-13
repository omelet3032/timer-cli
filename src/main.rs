mod timer;

use std::time::Duration;

use timer::Timer;
use timer::TimerCommand;
use timer::TimerState;
use tokio::io::{self, AsyncBufReadExt, BufReader};

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
    let mut reader = BufReader::new(io::stdin());
    let mut input = String::new();

    loop {
        input.clear();

        if reader.read_line(&mut input).await.is_ok() {
            if input.trim() == "1" {
                run_timer(&mut timer).await;
            }
        }
    }

    // run_timer(&mut timer).await;
}

async fn run_timer(timer: &mut Timer) {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut input = String::new();

    timer.start();
    println!("{}", timer);

    loop {
        tokio::select! {

            _ = tokio::time::sleep(Duration::from_secs(1)), if timer.state == TimerState::Working => {
                println!("{}", timer);

                timer.update();

                if timer.state == TimerState::Inactive {
                    println!("타이머가 종료되었습니다");
                }

            }

            res = reader.read_line(&mut input) => {
                if res.is_ok() {
                    let command = input.trim();

                    match command {
                        "pause" => {

                            // if let TimerState::Inactive = timer.state {
                            //     println!("Inactive : 일시정지를 할 수 없습니다");
                            // } else {
                            //    timer.pause();
                            //    println!("일시정지됨. (현재 시간: {})", timer);

                            // }

                            if timer.state == TimerState::Inactive {
                                println!("Inactive : 일시정지를 할 수 없습니다");
                            } else {

                                timer.pause();
                                println!("일시정지됨. (현재 시간: {})", timer);
                            }
                            }

                        }
                        "start" => {
                            timer.start();
                            println!("다시 시작!");
                            println!("{}", timer);
                        }
                        "reset" => {
                            timer.reset();
                            println!("초기화됨: {}", timer);
                        }
                        "exit" => {
                            println!("타이머 종료");
                            break;
                        }
                        _ => println!("알 수 없는 명령: {}", command),
                    }
                }
                input.clear();
            }


        }
    }
}

// setting
async fn run_setting() {}
