#![allow(dead_code, unused_variables)]
//use rand::prelude::*;
use rand::Rng;
use rand::prelude::SliceRandom;
use image::{RgbImage, Rgb};

#[derive(Clone, Copy, PartialEq)]  
pub enum CellType{
    Wall,
    Path,
    Start,
    End
}

pub struct Palette {
    pub wall: Rgb<u8>,
    pub path: Rgb<u8>,
    pub start: Rgb<u8>,
    pub end: Rgb<u8>
}

pub struct Maze {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>
}

#[derive(Clone, Copy, PartialEq)]  
pub struct Cell {
    /*Generic*/
    pub cell_type: CellType,
    
    /*DFS*/
    pub visited: bool,
}

impl Palette {
    pub fn default()-> Self {
        Palette {
            wall: Rgb([34, 40, 49]), /*Blackish*/
            path: Rgb([233, 227, 223]), /*Off White*/
            start: Rgb([70,92,136]), /*Blue*/
            end: Rgb([225,122,48]), /*Orange*/
        }
    }

    pub fn color_for(&self, cell_type: CellType) -> Rgb<u8> {
        match cell_type{
            CellType::Wall => self.wall,
            CellType::Path => self.path,
            CellType::Start => self.start,
            CellType::End => self.end
        }
    }
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {

        assert!(
            width % 2 == 1 && height % 2 == 1,
            "Maze dimensions must be odd"
        );

        let cell = Cell { cell_type: CellType::Wall , visited: false };
        let grid = vec![vec![cell; width]; height];
        Maze { width, height, grid }
    }

    pub fn generate_random(&mut self){
        let mut rng = rand::thread_rng();

        for (y,row) in self.grid.iter_mut().enumerate(){
            for (x,cell) in row.iter_mut().enumerate(){
                if x == 0 || y == 0 || y == self.height - 1 || x == self.width - 1{
                    cell.cell_type = CellType::Wall;
                }else{
                    cell.cell_type = if rng.gen_bool(0.7){ CellType::Wall } else { CellType::Path }
                }
            }
        }
    }

    pub fn generate_dfs_stack(&mut self) {
        println!("Generating Maze....");
        let TOTAL_CELLS = (self.height/2) * (self.width/2);
        let mut visited_count: usize = 0;
        
        let mut rng = rand::thread_rng();

        let (x, y) = (
            rng.gen_range(1..self.width / 2) * 2 - 1,
            rng.gen_range(1..self.height / 2) * 2 - 1,
        );
    
        let mut stack = vec![(x, y)];
    
        self.grid[y][x].visited = true;
        self.grid[y][x].cell_type = CellType::Path;
    
        while let Some((x, y)) = stack.pop() {
            let mut directions = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
            directions.shuffle(&mut rng);
    
            for (dx, dy) in directions {
                let nx = x as isize + dx * 2;
                let ny = y as isize + dy * 2;
    
                if nx > 0 && ny > 0 && nx < (self.width - 1) as isize && ny < (self.height - 1) as isize {
                    let ux = nx as usize;
                    let uy = ny as usize;
    
                    if !self.grid[uy][ux].visited {
                        let wall_x = (x as isize + dx) as usize;
                        let wall_y = (y as isize + dy) as usize;
                        self.grid[wall_y][wall_x].cell_type = CellType::Path;
    
                        self.grid[uy][ux].visited = true;
                        self.grid[uy][ux].cell_type = CellType::Path;
                        visited_count += 1;

                        if visited_count % 100_000 == 0 {
                            let prog = visited_count as f32 / TOTAL_CELLS as f32;
                            println!("{:.2}%", prog * 100.0);
                        }
    
                        stack.push((x, y));
                        stack.push((ux, uy));
                        break; 
                    }
                }
            }
        }

        self.set_start_stop_cells();
    }
    

    pub fn generate_dfs_rec(&mut self, start: Option<(usize,usize)>){

        let mut rng = rand::thread_rng();

        let (x, y) = match start {
            Some(coords) => coords,
            None => (
                rng.gen_range(1..self.width / 2) * 2 - 1,
                rng.gen_range(1..self.height / 2) * 2 - 1,
            ),
        };

       self.grid[y][x].visited = true;
       self.grid[y][x].cell_type = CellType::Path;

        
        let mut directions: Vec<(isize,isize)> = vec![
            (1,0), //North
            (-1,0), //South
            (0,1), //East
            (0,-1) //West
        ];

        directions.shuffle(&mut rng);

        for (dy,dx) in directions{
            let nx = x as isize + dx * 2;
            let ny = y as isize + dy * 2;

            if  nx > 0 && nx < (self.width - 1) as isize &&
                ny > 0 && ny < (self.height - 1) as isize {
              let ux = nx as usize;
              let uy = ny as usize;
              
                if !self.grid[uy][ux].visited {
                    let wall_x = (x as isize + dx) as usize;
                    let wall_y = (y as isize + dy) as usize;
                    self.grid[wall_y][wall_x].cell_type = CellType::Path;

                    self.generate_dfs_rec(Some((ux, uy)));
                }   
            }
        }

    }

    pub fn print_console(&mut self){
        for row in self.grid.iter(){
            for cell in row.iter(){
                print!("{}", 
                    if cell.cell_type == CellType::Wall {"#"} 
                    else if cell.cell_type == CellType::Path {" "}
                    else if cell.cell_type == CellType::Start {"O"}
                    else if cell.cell_type == CellType::End {"X"}
                    else {"?"}); /*For Unknown*/
            }
            println!("");
        }
    }

    pub fn save_maze_image(&mut self){
        println!("Generating Maze Image.");
        const CELL_SIZE: u32= 10; //10x10px per cell

        let palette: Palette = Palette::default();

        let mut img = RgbImage::new(self.width as u32 * CELL_SIZE, self.height as u32 * CELL_SIZE);

        for (y,row) in self.grid.iter().enumerate(){
            for (x,cell) in row.iter().enumerate(){
                let pixel = palette.color_for(cell.cell_type);

                for dy in 0..CELL_SIZE{
                    for dx in 0..CELL_SIZE{
                        img.put_pixel(
                            x as u32 * CELL_SIZE + dx,
                            y as u32 * CELL_SIZE + dy,
                            pixel,
                        );
                    }
                }
            }
        }
        img.save("maze.png").unwrap()
    }

    pub fn set_start_stop_cells(&mut self){

        let mut rng = rand::thread_rng();

        let (start_x, start_y) = (
            0,
            rng.gen_range(1..self.height - 1),
        );

        let (end_x, end_y) = (
            (self.width - 1) as usize,
            rng.gen_range(1..self.height - 1),
        );

        //verify valid this sucks but its 11pm
        if  self.grid[start_y][start_x + 1].cell_type == CellType::Wall || 
            self.grid[end_y][(end_x as isize - 1) as usize ].cell_type == CellType::Wall 
        {
            self.set_start_stop_cells();
        }else{
            self.grid[start_y][start_x].cell_type = CellType::Start;
            self.grid[end_y][end_x].cell_type = CellType::End;
        }
    }

}