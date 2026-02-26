작성하신 코드는 `tokio::select!`와 `BufReader`를 활용해 비동기 입출력을 아주 잘 구현하셨습니다! CLI 앱으로서의 구조가 탄탄하네요.

더 깔끔하고 유지보수하기 좋은 코드를 위한 **리팩토링 포인트 3가지**를 제안해 드립니다.

---

### 1. `run_timer` 내부의 출력 로직 개선

현재 타이머가 매초마다 `println!`으로 찍히는데, 터미널 화면이 아래로 계속 밀리는 현상이 발생할 겁니다. `\r`(커서 맨 앞으로)이나 ANSI 이스케이프 코드를 사용하면 한 줄에서 숫자가 바뀌는 세련된 UI를 만들 수 있습니다.

```rust
// 수정 전
println!("{}", timer);

// 리팩토링 제안: 한 줄 업데이트
use std::io::{Write, stdout};

print!("\r⏳ 현재 남은 시간: {}   ", timer); // \r로 커서를 맨 앞으로 보냄
stdout().flush().unwrap();

```

---

### 2. 에러 핸들링과 `CustomError` 확장

현재 `CustomError`가 하나뿐인데, 이를 조금 더 보완하여 입력값 파싱 실패 등을 명확히 구분하면 좋습니다. 또한, `thiserror` 크레이트를 사용하지 않더라도 `std::error::Error` 트레이트를 구현해주는 것이 정석입니다.

```rust
#[derive(Debug)]
enum TimerError {
    InputError(String),
    IoError(std::io::Error),
}

impl Display for TimerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerError::InputError(msg) => write!(f, "⚠️ 잘못된 입력: {}", msg),
            TimerError::IoError(e) => write!(f, "🚨 IO 에러: {}", e),
        }
    }
}

impl std::error::Error for TimerError {} // 표준 에러 트레이트 구현

```

---

### 3. `run_setting` 코드의 중복 제거 (매치 암등 제거)

`TimerDuration` 열거형과 `Duration`으로 변환하는 로직을 하나로 합치면 `run_setting` 함수가 훨씬 간결해집니다.

```rust
impl TimerDuration {
    // enum 자체에서 Duration을 반환하는 메서드 추가
    fn as_duration(&self) -> Duration {
        match self {
            TimerDuration::A30 => Duration::from_secs(30 * 60),
            TimerDuration::B60 => Duration::from_secs(60 * 60),
            TimerDuration::C90 => Duration::from_secs(90 * 60),
        }
    }
}

// run_setting 내부가 훨씬 깔끔해집니다
if let Ok(duration_enum) = parse_command::<TimerDuration>(&input) {
    return Ok(duration_enum.as_duration());
}

```

---

### 4. 코드 가독성 팁 (Small Wins)

* **`Duration` 표현:** `25 * 60` 보다는 `Duration::from_secs(25 * 60)`을 상수(`const`)로 빼두면 관리하기 좋습니다.
* **`handle_timer_command`의 반환형:** `Option<TimerCommand>` 대신 `bool` (루프 종료 여부)을 반환하거나, 좀 더 명확한 `ControlFlow` 같은 커스텀 enum을 쓰면 읽는 사람이 의도를 파악하기 더 쉽습니다.

### 🛠️ 리팩토링 후 기대 효과

이렇게 수정하면 **`main.rs`**는 전체적인 흐름 제어에 집중하고, 각 데이터 타입(`Timer`, `TimerDuration`)이 자기 할 일을 스스로 하는 **객체지향적인 Rust 코드**에 가까워집니다.

특히 **한 줄 업데이트 UI**는 `timer-cli`를 배포했을 때 사용자들이 "오, 꽤 괜찮은 앱인데?"라고 느끼게 만드는 큰 차이를 만들어낼 거예요.

이 리팩토링 제안 중에서 가장 먼저 적용해보고 싶은 부분이 있으신가요? 적용하시다가 막히는 문법이 있으면 바로 말씀해 주세요!