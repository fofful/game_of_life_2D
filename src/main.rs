use macroquad::prelude::*;
use std::{thread, time, env};
use std::collections::VecDeque;


fn window_conf() -> Conf {
    Conf {
        window_width: 1000,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let h: f32 = 10.0;
    let w: f32 = 10.0;
    let mut cells: Vec<[i16;3]> = vec![[50, 40, 1], [51, 40, 1], [52, 40, 1],[50, 39, 1], [50, 38, 1], [50, 37, 1], [53, 40, 1], [54, 40, 1], [55, 40, 1], [59, 40, 1], [58, 40, 1], [57, 40, 1],
                                        [53, 41, 1], [54, 42, 1], [55, 43, 1], [59, 44, 1], [58, 45, 1], [57, 46, 1]];
    let death_rule = [0,1,4,5,6,7,8];
    let live_rule = [2,3];
    let life_rule = [3];
    //Vec<(i16, i16)>
    loop {
        clear_background(WHITE);

        //drawing board
        for cell in cells.iter(){
            draw_rectangle((cell[0]*10) as f32, (cell[1]*10) as f32, w, h, BLACK);
        }

        //generating dead cells around the live cells
        for i in 0..cells.len(){
            for j in -1..2{
                for k in -1..2{
                    if cells.contains(&[cells[i][0]+j, cells[i][1]+k, 0]){}
                    else if cells.contains(&[cells[i][0]+j, cells[i][1]+k, 1]){}
                    else {
                        cells.push([cells[i][0]+j, cells[i][1]+k, 0]);
                    }
                }
            } 
        }

        //mutating the lifestates
        let mut life_state = VecDeque::new();
        for cell in cells.iter(){
            life_state.push_back(cell[2]);
        }

        
        //evaluating next generation
        for i in 0..cells.len(){
            let mut neighbour_count: i8 = 0;

            

            for j in 0..cells.len(){
                if cells[j][2] == 0{
                    continue;
                }
                if i == j{
                    continue
                }
                else{
                    let a = cells[i][0] - cells[j][0];
                    let b = cells[i][1] - cells[j][1];
                    if (a == -1 || a == 0 || a == 1 ) && (b == -1 || b == 0 || b == 1){
                        neighbour_count += 1;
                    }
                }
            //println!("{:?} cell, {:?} compared to cell, {} neighbours.", cells[i], cells[j], neighbour_count);
            }

            if life_rule.contains(&neighbour_count) && (cells[i][2] == 0){
                life_state[i] = 1;
            }
            else if live_rule.contains(&neighbour_count) && (cells[i][2] == 1){
                life_state[i] = 1;
            }
            else if death_rule.contains(&neighbour_count){
                life_state[i] = 0;
            }
        }

        //update life state for cells
        for i in 0..cells.len(){
            cells[i][2] = life_state[i];
        }

        //killing cells
        cells.retain(|&cell| cell[2] == 1);

        thread::sleep(time::Duration::from_millis(100));
        next_frame().await
    }
}