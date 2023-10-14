use std::cmp;
use futures::StreamExt;
use yew::{html, Component, AttrValue, Context, Html, classes};
use crate::services::{start_game_tick};
const GRID_HEIGHT: u8 = 30;
const GRID_WIDTH: u8 = 30;
const TICK_TIME: u64 = 100;

pub struct GameGridComponent{
    x: u8,
    y: u8,
}
pub enum Msg {
    GameTicked(())
}


enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl GameGridComponent {
    fn handle_dir_change(&mut self, dir: Direction) {
        match dir {
            Direction::UP => {
                self.y = cmp::max(self.y - 1, 0);
            }
            Direction::DOWN => {
                self.y = cmp::min(self.y + 1, GRID_HEIGHT - 1);
            }
            Direction::LEFT => {
                self.x = cmp::max(self.x - 1, 0);
            }
            Direction::RIGHT => {
                self.x = cmp::min(self.x + 1, GRID_WIDTH - 1);
            }
        }
    }
}

impl Component for GameGridComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let game_tick = start_game_tick(TICK_TIME);
        ctx.link().send_stream(game_tick.map(Msg::GameTicked));

        Self {
            x: 0,
            y: 0,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GameTicked(_) => {
                self.handle_dir_change(Direction::RIGHT);
            }
        }

        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <div>
                { for (0..GRID_HEIGHT).map(|row| {
                    html! {
                        <div class="row" key={row}>
                            { for (0..GRID_WIDTH).map(|column| {
                                html! {
                                    <div key={column} class={classes!(
                                        "cell",
                                        if self.x == column && self.y == row {
                                            "cell--active"
                                        } else {
                                            ""
                                        }
                                    )}></div>
                                }
                            })}
                        </div>
                    }
                })}
            </div>
        )
    }
}