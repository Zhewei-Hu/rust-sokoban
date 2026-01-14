#![allow(unused)]
fn main() {
    // resources.rs
    impl Display for GameplayState {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str(match self {
                GameplayState::Playing => "Playing",
                GameplayState::Won => "Won",
            })?;
            Ok(())
        }
    }
}
