mod maze;
use crate::maze::Maze;

fn main(){
    let mut maze = Maze::new(125,51);

    maze.generate_dfs_stack();
    maze.save_maze_image();
    //maze.print_console();
    //maze.display();
}