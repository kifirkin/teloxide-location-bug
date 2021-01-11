use crate::dialogue::Dialogue;
use teloxide::prelude::*;
use teloxide_macros::teloxide;

pub struct ReceiveLocationState;

#[teloxide(subtransition)]
async fn receive_location(
    _state: ReceiveLocationState,
    cx: TransitionIn,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer_str(format!("Location: {}", ans)).await?;
    exit()
}
