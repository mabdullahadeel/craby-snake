use std::cmp;
use futures::StreamExt;
use yew::{html, Component, Context, Html, classes, KeyboardEvent};
use crate::services::{start_game_tick};

type HNum = i8;

const BOUNDARY_THICKNESS: HNum = 1;
const GRID_HEIGHT: HNum = 30;
const GRID_WIDTH: HNum = 30;
const GRID_OFFSET: HNum = BOUNDARY_THICKNESS * 2;
const TICK_TIME: u64 = 100;

pub struct GameGridComponent{
    x: HNum,
    y: HNum,
    current_direction: Direction,
    score: u64,
    paused: bool,
}
pub enum Msg {
    GameTicked(()),
    HandleKeyboardEvent(KeyboardEvent),
    RestartGame(()),
    HandlePause(()),
}


enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl GameGridComponent {
    fn move_up(&mut self) {
        self.y = cmp::max(self.y - 1, 0);
    }
    fn move_down(&mut self) {
        self.y = cmp::min(self.y + 1, GRID_HEIGHT + GRID_OFFSET - 1);
    }
    fn move_left(&mut self) {
        self.x = cmp::max(self.x - 1, 0);
    }
    fn move_right(&mut self) {
        self.x = cmp::min(self.x + 1, GRID_WIDTH + GRID_OFFSET - 1);
    }
    fn update_direction(&mut self, dir: Direction) {
        self.current_direction = dir;
    }
    fn update_pause(&mut self, pause: bool) {
        self.paused = pause;
    }
    fn handle_keydown(&mut self, event: KeyboardEvent) {
        match event.key().as_str() {
            "ArrowUp" => self.update_direction(Direction::UP),
            "ArrowDown" => self.update_direction(Direction::DOWN),
            "ArrowLeft" => self.update_direction(Direction::LEFT),
            "ArrowRight" => self.update_direction(Direction::RIGHT),
            " " => self.update_pause(!self.paused), // spacebar
            _ => {}
        }
    }
    fn handle_tick(&mut self) {
        match self.current_direction {
            Direction::UP => self.move_up(),
            Direction::DOWN => self.move_down(),
            Direction::LEFT => self.move_left(),
            Direction::RIGHT => self.move_right(),
        }
    }
    fn is_game_over(&self) -> bool {
        is_boundary(self.x, self.y)
    }
    fn increment_score(&mut self) {
        self.score += 1;
    }
    fn restart(&mut self) {
        self.x = 1;
        self.y = 1;
        self.current_direction = Direction::RIGHT;
        self.score = 0;
        self.paused = false;
    }
}

fn is_boundary(x: HNum, y: HNum) -> bool {
    x < BOUNDARY_THICKNESS || x >= GRID_WIDTH + BOUNDARY_THICKNESS || y < BOUNDARY_THICKNESS || y >= GRID_HEIGHT + BOUNDARY_THICKNESS
}

impl Component for GameGridComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let game_tick = start_game_tick(TICK_TIME);
        ctx.link().send_stream(game_tick.map(Msg::GameTicked));

        Self {
            x: 1,
            y: 1,
            current_direction: Direction::RIGHT,
            score: 0,
            paused: true,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GameTicked(_) => {
                if self.paused {
                    return false; // don't re-render if paused
                }
                if self.is_game_over() {
                    self.update_pause(true);
                } else {
                    self.handle_tick();
                    self.increment_score();
                }
            }
            Msg::HandleKeyboardEvent(event) => {
                self.handle_keydown(event);
            }
            Msg::RestartGame(_) => {
                self.restart();
            }
            Msg::HandlePause(_) => {
                self.update_pause(!self.paused);
            }
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let handle_keydown = ctx.link().callback(|e: KeyboardEvent| {
            Msg::HandleKeyboardEvent(e)
        });
        let handle_restart = ctx.link().callback(|_| {
            Msg::RestartGame(())
        });
        let handle_unpause = ctx.link().callback(|_| {
            Msg::HandlePause(())
        });


        html!(
            <div tabIndex="0" onkeydown={handle_keydown}>
            {if self.is_game_over() {
                html! {
                    <div>
                        <p>{ "Game Over" }</p>
                        <button onclick={handle_restart}>{ "Restart" }</button>
                    </div>
                }
            } else {html!{
                <button onclick={handle_unpause}>{ if self.paused {"Resume"} else {"Pause"} }</button>
            }}}
            <p>{ format!("scrore: {}", self.score) }</p>
                { for (0..GRID_HEIGHT + GRID_OFFSET).map(|row| {
                    html! {
                        <div class="row" key={row}>
                            { for (0..GRID_WIDTH + GRID_OFFSET).map(|column| {
                                html! {
                                    <div key={column} class={classes!(
                                        "cell",
                                        if self.x == column && self.y == row {
                                            "cell--active"
                                        } else {
                                            ""
                                        },
                                        if is_boundary(column, row) {
                                            "cell--boundary"
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