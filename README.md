# ts1-game-2048-rust
러스트 스터디 그룹 팀 프로젝트1: 2048

- ROOT
    - /entity
        - [`defines.rs`](http://defines.rs) : 공통 속성값들 정의
        - 게임 전반적으로 사용되는 enum 과 impl
        - `mod.rs`
    - /component
        - bevy 에서 정의한 `#[derive(Component)]` 들
        - `mod.rs`
    - /system
        - component 들을 제어하며 게임에 영향을 주는 함수들
        - 순수 게임 전반 시스템을 제어하기 위한 함수들
        - `mod.rs`
    - /ui
        - 와꾸
        - `mod.rs`
    - `main.rs`
    - `lib.rs`
