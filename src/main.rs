use yew::Renderer;
use crate::game_grid::GameGridComponent;

mod game_grid;
fn main() {
    Renderer::<GameGridComponent>::new().render();
}