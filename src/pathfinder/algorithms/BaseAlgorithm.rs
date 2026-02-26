use crate::pathfinder::grid::Grid;

pub trait BaseAlgorithm{
    fn solve(grid: Grid, diagonal_allowed: bool) -> Grid;
}