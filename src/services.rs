use std::time::Duration;
use futures::Stream;
use yew::platform::time::interval;

pub fn start_game_tick(ms: u64) -> impl Stream<Item = ()> {
    interval(Duration::from_millis(ms))
}