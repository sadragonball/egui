mod left_side_panel;
mod frame_history;

pub use left_side_panel::LeftSidePanel;
pub use frame_history::FrameHistory;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunMode {
    Reactive,
    Continuous
}

impl Default for RunMode {
    fn default() -> Self {
        RunMode::Reactive
    }
}