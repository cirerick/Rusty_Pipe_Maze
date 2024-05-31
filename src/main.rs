#[allow(non_snake_case)]
mod pipes;

#[path = "lib/cin.rs"]
mod cin;

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

    //output map details
    println!("\nSpawn Maps Paths:");
    map.print_contents();


    //spawn a map containing 4 paths and 12 bridges
    println!("Enter number to choose path.");
    let u_pos: u8 = match cin::cin_u8() { // for now just take a number between
        y if y < 1 || y > path_n + 1 => {
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

    println!("You have entered {}", u_pos);
}
