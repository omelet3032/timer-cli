mod timer;

use std::fmt::Display;
use std::fmt::Formatter;
use std::io::Write;
use std::io::stdout;
use std::str::FromStr;
use std::time::Duration;
use timer::Timer;
use timer::TimerCommand;
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut input = String::new();

    // let mut timer = Timer::new(Duration::from_secs(25 * 60));
    let mut timer = Timer::new(Duration::from_secs(5));

    println!("timer-cli start");

    loop {
        input.clear();
        println!("메뉴 : 1)️timer 2)setting");

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
                _ => {
                    println!("다시 입력해주세요");
                }
            }
        }
    }
}

async fn run_timer(timer: &mut Timer, reader: &mut BufReader<tokio::io::Stdin>) {
    let mut input = String::new();

    timer.start();

    print!("\r⏳ 현재 남은 시간: {}   ", timer); // \r로 커서를 맨 앞으로 보냄
    stdout().flush().unwrap();

    let mut tick = tokio::time::interval(Duration::from_secs(1));
    
    loop {
        tokio::select! {

            // _ = tick.tick(), if timer.is_working() => {
            _ = tick.tick() => {
                print!("\r⏳ 현재 남은 시간: {}   ", timer); // \r로 커서를 맨 앞으로 보냄
                stdout().flush().unwrap();
             }

             _ = tokio::time::sleep_until(timer.deadline().into()), if timer.is_working() => {
                print!("\r⏳ 현재 남은 시간: {}   ", timer); // \r로 커서를 맨 앞으로 보냄
                stdout().flush().unwrap();
                 timer.deactivate();
                 println!("\n타이머가 종료되었습니다");

            }

            res = reader.read_line(&mut input) => {
                if res.is_ok() {
                    if let Some(TimerCommand::Quit) = handle_timer_command(timer, &input) {
                        break;
                    }
                }
                input.clear();
            }
        }
    }
}

fn handle_timer_command(timer: &mut Timer, input: &str) -> Option<TimerCommand> {
    let command = parse_command::<TimerCommand>(input).ok()?;

    match command {
        // start와 restart를 구분할 필요가 있을듯
        // 타이머가 끝난 상황에서 s는 start, pause상태에서 s는 restart다.
        TimerCommand::Start => {
            if timer.is_working() {
                println!("작동중입니다");
            } else {
                timer.start();
                // tick.reset();
                // println!("다시 시작!");
                // println!("{}", timer);
            }
            None
        }
        TimerCommand::Pause => {
            if timer.is_inactive() {
                println!("Inactive : 일시정지를 할 수 없습니다");
            } else {
                timer.pause();
                println!("일시정지됨. (현재 시간: {})", timer);
            }
            None
        }
        TimerCommand::Reset => {
            timer.reset();
            println!("초기화됨: {}", timer);
            None
        }
        TimerCommand::Quit => {
            timer.reset();
            println!("메뉴로 돌아가기");
            Some(TimerCommand::Quit)
        }
    }
}

fn parse_command<T: FromStr>(input: &str) -> Result<T, ()> {
    input.trim().parse().map_err(|_| ())
}
// setting
/*
    setting의 역할
        work_duration을 사용자가 입력으로 설정한후 그 값을 main()에 전달
*/

#[derive(Clone, Debug)]
pub struct Error {
   kind: ErrorKind, 
}

impl Error {
    pub(crate) fn input<E: std::error::Error>(err: E) -> Error {
        Error {kind: ErrorKind::Input(err.to_string())}
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    Input(String),
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::Input(ref s) => write!(f, "{}", s),
        }
    }
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

async fn run_setting(reader: &mut BufReader<tokio::io::Stdin>) -> Result<Duration, TimerError> {
    let mut input = String::new();

    loop {
        println!("시간을 선택해주세요. (안할시 기본값 25분");
        println!("1) 30분 2) 60분 3) 90분");

        input.clear();

        match reader.read_line(&mut input).await {
            Ok(_) => match parse_command::<TimerDuration>(&input) {
                Ok(duration_enum) => {
                    let new_duration = match duration_enum {
                        TimerDuration::A30 => Duration::from_secs(30 * 60),
                        TimerDuration::B60 => Duration::from_secs(60 * 60),
                        TimerDuration::C90 => Duration::from_secs(90 * 60),
                    };
                    return Ok(new_duration);
                }
                Err(_) => {
                    println!("다시 입력해주세요");
                }
            },
            Err(_) => {
                return Err(TimerError::InputError);
            }
        }
    }
}
