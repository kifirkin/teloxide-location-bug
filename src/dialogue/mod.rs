mod states;

use crate::dialogue::states::{ReceiveLocationState, StartState};
use derive_more::From;
use teloxide_macros::Transition;

#[derive(Transition, From)]
pub enum Dialogue {
    Start(StartState),
    ReceiveLocation(ReceiveLocationState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}
