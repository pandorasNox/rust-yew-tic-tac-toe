
enum Msg {
  GridChange(usize, usize)
}

fn main() {
  let mut grid: [[i32;3]; 3] = [[0,0,0], [0,0,0], [0,0,0],];
  let msg = Msg::GridChange(0,0);

  match &msg {
    Msg::GridChange(i_row, i_col) => {
      grid[*i_row][*i_col] = 0
    }
  }
}
