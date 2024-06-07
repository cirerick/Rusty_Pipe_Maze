#[allow(non_snake_case)]
mod pipes;

#[path = "lib/cin.rs"]
mod cin;

use std::path;

use crate::pipes::resu;
use crate::resu::User;


//we will add the timer here, although it would be more idiomatic to express within pipes.rs or maybe an update/draw.rs
fn main() {
    //store path and bridge amount
    let path_n: u8 = 4;
    let bridge_n: u8 = path_n *  3;

    //store spawned map
    let map: pipes::Map<i32> = pipes::spawn_map::<i32>(path_n, bridge_n);
    //store ascii render
    let mut map_drawing: Vec<Vec<char>> = pipes::generate_map_drawing(&map, path_n, bridge_n);

    //output map
    println!("\nDrawing Map:\n");
    pipes::draw_map(&map_drawing, path_n, bridge_n); //could structure this -> put into struct


    //get user desired starting point
    println!("Enter number to choose path.");
    let u_path_m: u8 = match cin::cin_u8() { 
        y if y < 1 || y >= path_n + 1 => {
            loop {
                println!("Value {} out of scope. Please enter number within range in correlation to amount of paths.", y);
                match cin::cin_u8() {
                    inner_x if inner_x > 0 && inner_x < path_n + 1 => {break inner_x}
                    _ => {continue},
                }
            }
        },
        x => x,
    };

    //spawn user
    let mut resu: resu::User<i32> = User::new(&map.path_list[(u_path_m as usize) - 1], 4 * ((u_path_m as usize) - 1), map_drawing.len() - 1);

    /* 
        chart[row][col]
        row = y position
        col = x position
    */
    
    //get graphic that it will replace
    let mut prev_ascii: char = map_drawing[resu.ascii_pos.1][resu.ascii_pos.0];

    map_drawing[resu.ascii_pos.1][resu.ascii_pos.0] = '♥'; 
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    pipes::draw_map(&map_drawing, path_n, bridge_n);

    let mut running: bool = true; 

    while running {
        pipes::next_step(&mut map_drawing, &mut resu, &mut prev_ascii, &mut running, path_n, bridge_n);
    }
    
    //output map details
    println!("\nSpawn Maps Paths:");
    map.print_contents();
}
