use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use crate::pathfinder::algorithms::BaseAlgorithm::BaseAlgorithm;
use crate::pathfinder::grid::Grid;

struct AStar{
}

impl AStar{
    fn heuristic(cell: [i64; 2], end_node: [i64;2]) -> i64{
        (cell[0] - end_node[1]).abs() + (cell[1] - end_node[1]).abs()
    }
}
impl BaseAlgorithm for AStar {
    fn solve(mut grid: Grid, diagonal_allowed: bool) -> Grid{
        let start = grid.get_start();
        let end = grid.get_end();
        
        let mut g_costs: HashMap<[i64; 2], i64> = HashMap::new();
        let mut f_costs: HashMap<[i64;2], i64> = HashMap::new();
        let mut open_list: BinaryHeap<(Reverse<i64>, Vec<i64>)> = BinaryHeap::new();
        let directions: &[(isize, isize)];
        
        if (start == [-1,-1] || end == [-1,-1]){
            return grid;
        }
        

        if diagonal_allowed {
            directions = &[
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1), (0, 1),
                (1, -1), (1, 0), (1, 1)
            ]
        } else {
            directions = &[(-1, 0), (0, -1), (0, 1), (1, 0)]
        }
        
        // let cost = *f_costs.get(&cell).unwrap_or(&i32::MAX);
        // open_list.push((Reverse(cost), cell));
        g_costs.insert(start, 0);
        f_costs.insert(start, Self::heuristic(start, end));

        while !open_list.is_empty() {
            let current_cell = open_list.pop().unwrap();

            if !current_cell.1.eq(&start){
                grid.update_field(current_cell.1[0] as usize, current_cell.1[1] as usize, 3)
            } else if current_cell.1.eq(&end){
                break;
            }


        }
        //trace path for grid
        grid
    }
}