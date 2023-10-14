use yew::Renderer;
use crate::game_grid::GameGridComponent;

mod services;
mod game_grid;
fn main() {
    Renderer::<GameGridComponent>::new().render();
}