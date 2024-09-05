extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::video::Window;
use sdl2::rect::{Point,Rect};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub const LENGHT_CONST: i32 = 20;

pub struct MapGrid{
    pub grid_spacing: usize,
    pub width: u32, 
    pub height: u32
}

impl MapGrid{
    pub fn new(spacing: usize,width: u32,height: u32) -> Self{
        MapGrid{ 
            grid_spacing: spacing,
            width: width,
            height: height
        }
    }
    pub fn render(&self,canvas: &mut sdl2::render::Canvas<Window>){
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for x in (0..self.width).step_by(self.grid_spacing) {
            canvas.draw_line(Point::new(x as i32, 0), Point::new(x as i32, self.height as i32)).unwrap();
        }
        for y in (0..self.height).step_by(self.grid_spacing) {
            canvas.draw_line(Point::new(0, y as i32), Point::new(self.width as i32, y as i32)).unwrap();
        }
    }
}

fn load_map() -> Result<Vec<Vec<i32>>, String> {
    let file = File::open("life_map.txt").map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        // Split the line by whitespace and convert to integers
        let row: Vec<i32> = line
            .split_whitespace() // Split by spaces
            .map(|s| s.parse().unwrap()) // Parse each part to an integer
            .collect();
        matrix.push(row); // Add the row to the matrix
    }
    Ok(matrix)
}

pub fn render_cells(canvas: &mut sdl2::render::Canvas<Window>,map_of_cell: Vec<Vec<i32>>){
    for (row_index, row) in map_of_cell.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            let current_cell = Rect::new(col_index as i32*LENGHT_CONST as i32,row_index as i32*LENGHT_CONST as i32, LENGHT_CONST as u32,LENGHT_CONST as u32);
            if value == 1{
                canvas.set_draw_color(Color::RGB(0,100,0));
            }
            else if value == 0{
                canvas.set_draw_color(Color::RGB(255,255,255));
            }
            canvas.fill_rect(current_cell).ok().unwrap_or_default();
        }
    }
}

pub fn rules(state: i32,neighs:i32) -> i32{
    if state == 1{
        if neighs < 2{
            return 0;
        }
        else if neighs > 3{
            return 0;
        }
        else{
            return 1;
        }
    }
    else if state == 0{
        if neighs == 3{
            return 1;
        }
        else{
            return 0;
        }
    }
    0 
}

pub fn count_neighs(pos_x: i32,pos_y:i32,board: &Vec<Vec<i32>>)->i32{
    let mut count_neighs = 0;
    let directions = vec![-1,0,1];
    for dx in &directions{
        for dy in &directions{
            if *dx == 0 && *dy == 0{
                continue;
            }
            let mut row_neig = pos_x + dx;
            let mut col_neig = pos_y + dy;
            if row_neig > (board.len()-1).try_into().unwrap(){
                row_neig = 0;
            }
            else if row_neig < 0{
                row_neig = (board.len()-1).try_into().unwrap();
            }
            if col_neig > (board[0].len()-1).try_into().unwrap(){
                col_neig = 0;
            }
            else if col_neig < 0{
                col_neig = (board[0].len() - 1).try_into().unwrap();
            }
            count_neighs += board[row_neig as usize][col_neig as usize]
        }
    }
    count_neighs
}

fn reset_life_board(life_board: &mut Vec<Vec<i32>>) {
    let rows = life_board.len();
    let cols = if rows > 0 { life_board[0].len() } else { 0 };
    *life_board = vec![vec![0; cols]; rows];
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut list_of_cells = load_map()?;
    let num_rows = list_of_cells.len(); 
    let num_cols = list_of_cells[0].len();
    let window = video_subsystem.window("Conway", num_cols  as u32*LENGHT_CONST as u32, num_rows  as u32*LENGHT_CONST as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let (map_width, map_height) = canvas.output_size().unwrap();
    let my_grid = MapGrid::new(LENGHT_CONST as usize, map_width, map_height);
    // let mut my_cell = Cell::new(5,5);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let old_list_of_cells = list_of_cells.clone();
        reset_life_board(&mut list_of_cells);
        for (row_index, row) in old_list_of_cells.iter().enumerate() {
            for (col_index, &value) in row.iter().enumerate() {
                let neighs = count_neighs(row_index as i32,col_index as i32,&old_list_of_cells);
                list_of_cells[row_index][col_index] = rules(old_list_of_cells[row_index][col_index],neighs);
            }
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        render_cells(&mut canvas, list_of_cells.clone());
        my_grid.render(&mut canvas);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
    }

    Ok(())
}