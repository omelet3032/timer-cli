# timer-cli todo

## 기능 



## 리팩토링 
- [x] mut timer : Option보다는 초기에 Duration from 25 * 60을 주는건 어떨까? 
- [ ] run_timer match 분기문 별도 함수로 추출 (handle_command)
- [ ] timer.state == working => is_working 함수로 변경 (캡슐화)
- [ ]  입력 스트림 main으로 단일화
- [ ]  run_timer 책임분리


## 메모
1. CustomError는 run_setting의 반환 타입을 Result로 구현하기 위해 일부러 만든 enum
2. let mut timer에 바로 25 * 60을 부여해서 복잡한 로직을 없애긴 했지만 (ex. timer_opt:Option<Timer> get_or_insert_with) 어쨌든 Option 활용과 새로운 메서드를 학습할 수 있었다.      