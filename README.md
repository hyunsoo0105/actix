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

