// mod error;
mod timer;

use std::io::Write;
use std::io::stdout;
use std::ops::ControlFlow;
use std::time::Duration;
use tokio::io::{self, AsyncBufReadExt, BufReader};

use timer::{Timer, TimerCommand, TimerDuration};

#[tokio::main]
async fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut input = String::new();

    let mut timer = Timer::new(Duration::from_secs(5));

    println!("timer-cli start");

    loop {
        input.clear();
        println!("메뉴 : 1)️timer 2)setting 3)exit");

        if reader.read_line(&mut input).await.is_ok() {
            match input.trim() {
                "1" => {
                    run_timer(&mut timer, &mut reader).await;
                }
                "2" => {
                    let result = run_setting(&mut reader).await;
                    match result {
                        Ok(new_duration) => {
                            timer.change_duration(new_duration);
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                }
                "3" => {
                    break;
                }
                _ => {
                    println!("다시 입력해주세요");
                }
            }
        }
    }
    println!("bye~")
}

fn display_timer_msg(timer: &mut Timer) {
    print!("\r⏳ 현재 남은 시간: {}   ", timer);
    stdout().flush().unwrap();
}

async fn run_timer(timer: &mut Timer, reader: &mut BufReader<tokio::io::Stdin>) {
    let mut input = String::new();

    timer.start();

    display_timer_msg(timer);
    let mut tick = tokio::time::interval(Duration::from_secs(1));

    loop {
        tokio::select! {

            _ = tick.tick(), if timer.is_working() => {
                display_timer_msg(timer);
             }

             _ = tokio::time::sleep_until(timer.deadline().into()), if timer.is_working() => {
                display_timer_msg(timer);
                timer.deactivate();
                println!("\n타이머가 종료되었습니다");
            }

            result = reader.read_line(&mut input) => {
                if result.is_ok() {
                    if handle_timer_command(timer, &input).is_break() {
                        break;
                    }
                }
                input.clear();
            }
        }
    }
}

fn handle_timer_command(timer: &mut Timer, input: &str) -> ControlFlow<()> {
    // // let Ok(command) = input.trim().parse::<TimerCommand>() else {
    //     let err = crate::error::Error::msg(input.trim());
    //     eprintln!("{}", err);
    //     return ControlFlow::Continue(());
    // };

    let command = match input.trim().parse::<TimerCommand>() {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("{}", e);
            return ControlFlow::Continue(());
        }
    };

    // 반환 타입이 맞지 않는다. 
    // 추후 Result<>타입으로 변경후 main에서 에러 처리하는 방안을 고려해보자.
    // let command = input.trim().parse::<TimerCommand>().map_err(|e| {
    //     eprintln!("{}", e);
    //     ControlFlow::Continue(());
    // })?;

    match command {
        TimerCommand::Start => {
            if timer.is_working() {
                println!("작동중입니다");
            } else {
                timer.start();
            }
        }
        TimerCommand::Pause => {
            if timer.is_inactive() {
                println!("Inactive : 일시정지를 할 수 없습니다");
            } else {
                timer.pause();
                println!("일시정지됨. (현재 시간: {})", timer);
            }
        }
        TimerCommand::Reset => {
            timer.reset();
            println!("초기화됨: {}", timer);
        }
        TimerCommand::Quit => {
            timer.reset();
            println!("메뉴로 돌아가기");
            return ControlFlow::Break(());
        }
    }

    ControlFlow::Continue(())
}

async fn run_setting(reader: &mut BufReader<tokio::io::Stdin>) -> Result<Duration, std::io::Error> {
    let mut input = String::new();

    loop {
        println!("시간을 선택해주세요. (기본값 25분)");
        println!("1) 30분 2) 60분 3) 90분");

        input.clear();

        reader.read_line(&mut input).await?;

        let duration_enum = match input.trim().parse::<TimerDuration>() {
            Ok(v) => {
                println!("'{}'을 선택하셨습니다.", v);
                v
            },
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        let new_duration = match duration_enum {
            TimerDuration::M30 => Duration::from_secs(30 * 60),
            TimerDuration::M60 => Duration::from_secs(60 * 60),
            TimerDuration::M90 => Duration::from_secs(90 * 60),
        };

        return Ok(new_duration);
    }
}
