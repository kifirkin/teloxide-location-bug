use crate::dialogue::Dialogue;
use teloxide::{
    prelude::*,
    types::{ButtonRequest, KeyboardButton, ReplyKeyboardMarkup},
};
use teloxide_macros::teloxide;

use super::ReceiveLocationState;

pub struct StartState;

#[teloxide(subtransition)]
async fn start(_state: StartState, cx: TransitionIn, _ans: String) -> TransitionOut<Dialogue> {
    let location_kbd = ReplyKeyboardMarkup::default()
        .append_row(vec![
            KeyboardButton::new("Send Location 📍").request(ButtonRequest::Location)
        ])
        .one_time_keyboard(true)
        .resize_keyboard(true);

    cx.answer("Send me location please!")
        .reply_markup(location_kbd)
        .send()
        .await?;

    next(ReceiveLocationState)
}
