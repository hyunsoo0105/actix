# 들어가며

RusT에 다양한 Web Framework가 있어서 고민을 했다.

* Actix <br>
    1. 유명하며 강력한 프레임워크이다.
    2. 동시성이나 성능 측면에서 Rocket보다 좋다고 한다.
* Rocket <br> 
    1. 유명하며 쉽게 작성할수 있는 프레임워크이다.
    2. 보안, 유연성, 기능을 챙기며 쉽게 개발할 수 있다.
    3. 개발자 커뮤니티가 크다.
* Amux  <br>
    1. 비동기적이고 확장 가능하며 유지 관리 가능한 웹 애플리케이션을 구축하기 위한 기반을 제공하는 데 중점을 둔 웹 프레임워크이다.
    2. Tokio를 만든 팀에서 만든 프레임워크이다.

위의 내용은 검색하며 수집한 정보이며 위의 확인한 프레임워크 외에도 다양한 프레임워크가 존재한다.

나는 Actix을 사용해 보기로 결정했다. 그 이유는 가장 버전이 높아 선택했다.

> [Actix](https://actix.rs/) 웹사이트를 참고하며 일단 해보겠다.

<br>

# Cargo 프로젝트 생성방법

`cargo new [프로젝트명]`

사용하여 새로운 프로잭트 폴더 생성

`cargo init`

사용하여 해당 폴더에 프로젝트 생성

<br>



# Cargo 실행방법

`cargo run`

상단 명령어를 통해 실행

`cargo watch -x run`

상단 명령어를 통해 auto reload 실행 (cargo-watch 설치 필요)

------

### 1. Actix 실행해보기

[Cargo.toml](./Cargo.toml) dependencies에 actix-web을 작성

src 폴더에 [main.rs](./src/main.rs) 작성

> 서버를 실행하여 서버 동작 확인
<br>Api를 호출하여 응답 확인 <br>

### 2. State 와 AppData

참고 : [state 폴더](./src/state)

[state.rs](./src/state/state.rs)
* app_data를 사용하여 서버가 실행 될 대 값을 설정해 놓을수있다.
* mutex로 가변되는 변수 설정 가능
* state_config 함수처럼 ServiceConfig를 받아 서버 설정 가능

[mod.rs](./src/state/mod.rs)
* 해당 폴더 내 코드를 다른데서 사용가능하게 mod를 사용해 모듈로  state를 설정
* 이방법 말고도 다른 방법도 있음 추후 사용예정

[state.rs](./src/state/state.rs)
* ServiceConf를 통해 원하는 서버의 설정을 추가 할 수 있음
* app_data를 통해 필요한 데이터를 서버 실행시 설정할 수 있음
* Mutex를 사용하여 공유데이터를 보호하는 변수 생성
* web::Data를 사용하여 스레드간 안전하게 참조가능한 공간 생성
* 가변 state를 사용할때는 먼저 web::Data를 선언 후 clone으로 app_date에 등록한다.
  * 이유 : web::Data는 [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)를 사용하는데 두개를 생성하지 않기 위해선 데이터 등록 전 생성해야함
* web::scope를 사용해 접두사를 설정 가능

[main.rs](./src/main.rs)
* 선언한 state의 state_config 함수를 state::state::state_config와 같이 config에 사용
* worker를 통해 멀티 스레드 설정
  * 해당 설정시 expect를 사용하여 에러 컨트롤을 해줘야 실행이 가능
