use std::os::linux::raw::stat;

pub struct Grid{
    field: Vec<Vec<u8>>
}

impl Grid{
    pub fn generate_grid(n: usize, m: usize) -> Self{
        let mut field: Vec<Vec<u8>> = Vec::new();

        for i in 0..n{
            for j in 0..m{
                field[i][j] = 0;
            }
        }

        Self{
            field
        }
    }

    pub fn update_field(&mut self, row: usize, col: usize, state: u8){
        if row < self.field.len() && col < self.field[row].len() {
            self.field[row][col] = state;
        }
    }

    pub fn get_state_of_field_cord(&self, row: usize, col: usize) -> u8{
        if row < self.field.len() && col < self.field[row].len() {
            return self.field[row][col]
        }
        0
    }

    pub fn get_start(&self) -> [i64; 2] {
        for row in 0..self.field.iter().len(){
            for col in 0..self.field[row].iter().len(){
                if self.field[row][col] == 1{
                    return [row as i64, col as i64];
                }
            }
        }
        [-1,-1]
    }

    pub fn get_end(&self) -> [i64; 2] {
        for row in 0..self.field.iter().len(){
            for col in 0..self.field[row].iter().len(){
                if self.field[row][col] == 2{
                    return [row as i64, col as i64];
                }
            }
        }
        [-1,-1]
    }
}