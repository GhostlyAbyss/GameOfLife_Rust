use crate::game_of_life::main::GameOfLifeApp;

pub enum AppScreen{
    StartMenu,
    GameOfLife(GameOfLifeApp)
}