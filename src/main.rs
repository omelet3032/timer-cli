mod timer;

use std::str::FromStr;
use std::time::Duration;

use timer::Timer;
use timer::TimerCommand;
use timer::TimerState;
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    // let mut work_duration = Duration::from_secs(10);

    // let mut timer = Timer::new(work_duration);

    let mut reader = BufReader::new(io::stdin());
    let mut input = String::new();
    
    let mut timer_opt:Option<Timer> = None;

    loop {
        println!("timer-cli start");
        println!("메뉴 : 1)️timer 2)setting");
        input.clear();

        if reader.read_line(&mut input).await.is_ok() {
            match input.trim() {
                "1" => {
                    // let mut work_duration = Duration::from_secs(10);

                    // let mut timer = Timer::new(Duration::from_secs(25*60));
                    let mut timer = timer_opt.get_or_insert_with(|| Timer::new(Duration::from_secs(25*60)));
                    run_timer(&mut timer).await;
                }
                "2" => {
                    let result = run_setting().await;
                    match result {
                        Ok(new_duration) => {
                            // if let Some(ref mut timer) = timer_opt {
                            if let Some(timer) = &mut timer_opt {
                                timer.change_duration(new_duration);
                                // timer_opt = Some(timer);
                            } else {
                                timer_opt= Some(Timer::new(new_duration));
                            }
                            
                        }
                        Err(e) => {
                            println!("에러 : {:?}", e);
                        }
                    }
                }
                _ => {
                    println!("다시 입력해주세요");
                }
            }
        }
    }
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
                            if timer.state == TimerState::Inactive {
                                println!("Inactive : 일시정지를 할 수 없습니다");
                            } else {
                                timer.pause();
                                println!("일시정지됨. (현재 시간: {})", timer);
                            }
                        },
                        "start" => {
                            timer.start();
                            println!("다시 시작!");
                            println!("{}", timer);
                        },
                        "reset" => {
                            timer.reset();
                            println!("초기화됨: {}", timer);
                        },
                        "quit" => {
                            println!("메뉴로 돌아가기");
                            timer.reset();
                            return;
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
/*
    setting의 역할
        work_duration을 사용자가 입력으로 설정한후 그 값을 main()에 전달
*/

#[derive(Debug)]
enum CustomError {
    InputError,
}

enum TimerDuration {
    A30,
    B60,
    C90,
}

impl FromStr for TimerDuration {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(TimerDuration::A30),
            "2" => Ok(TimerDuration::B60),
            "3" => Ok(TimerDuration::C90),
            _ => Err(()),
        }
    }
}

async fn run_setting() -> Result<Duration, CustomError> {
    println!("시간을 선택해주세요. (안할시 기본값 25분");
    println!("1) 30분 2) 60분 3) 90분");

    let mut reader = BufReader::new(io::stdin());
    let mut input = String::new();

    loop {
        input.clear();

        if reader.read_line(&mut input).await.is_err() {
            return Err(CustomError::InputError);
        }

        if reader.read_line(&mut input).await.is_ok() {
            // parse()에 Result<F, F::Err>로 정의되어있다. 왜 이걸 제대로 보지 않았을까..
            match input.trim().parse::<TimerDuration>() {
                Ok(duration_enum) => {
                    let new_duration = match duration_enum {
                        TimerDuration::A30 => Duration::from_secs(30 * 60),
                        TimerDuration::B60 => Duration::from_secs(60 * 60),
                        TimerDuration::C90 => Duration::from_secs(90 * 60),
                    };
                    // work_duration = new_duration;
                    return Ok(new_duration);
                }
                Err(_) => {
                    println!("다시 입력해주세요.");
                }
            }
        }
    }
}
