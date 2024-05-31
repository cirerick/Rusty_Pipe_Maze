#[path = "lib/math.rs"]
mod math;

#[path = "lib/cin.rs"]
mod cin;

#[path = "resu.rs"]
mod resu;

use std::alloc::alloc;
use std::alloc::handle_alloc_error;
use std::ops::DerefMut;
use std::ptr;
use std::alloc;

/*
Basically Mario Party Pipe Maze
Structured around linked list
Then draw and animate it using cmd

Using Too Many Linked List as the learning permise
*/

//will make drawing it out easier
#[derive(Debug)]
enum Side{
    R, //right
    L, //left
    None, 
}

#[derive(Debug)]
struct Node<T> {
    //data memory based
    id: u8, //incremented with each node spawn
    data: T,

    //memory location based
    on_path: u8, 
    bridged: *mut Node<T>, //used to have two nodes point to each other
    side: Side, 
    next: *mut Node<T>, //point to next on path
    
}

#[derive(Debug)]
struct Path<T> {
    id: u8, //incremented with each node spawn
    start: *mut Node<T>, //will act as the 'head' of the list
    goal: Option<*mut Node<T>>, //if null/none no goal //act as tail of list
}

#[derive(Debug)]
pub struct Map<T> {
    path_list: Vec<Path<T>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        return Node {
            id: 0,
            data: data,
            on_path: 0,
            bridged: std::ptr::null_mut(),
            side: Side::None,
            next: std::ptr::null_mut(), 
        }
    }

    //used to be new() method, now set to default
    fn default(&mut self, id: u8, data: T, path_id: u8) {
        self.id = id;
        self.data = data;
        self.on_path = path_id;
        self.bridged = std::ptr::null_mut();
        self.side = Side::None;
        self.next = std::ptr::null_mut();
    }

    //we could probably save most recently returned pointer
    fn get_node(&mut self, id: u8) -> &mut Node<T> {
        if self.id != id && !(self.next.is_null()) {
            return unsafe {(*self.next).get_node(id)}
        } else {
            return self
        }
    } //what happens if id does not exist in list //it will return tail node

    fn set_next(&mut self, id: u8, data: T, path_id: u8) { //rewrite as recursive function
        unsafe {
            if !(self.next.is_null()) {
                return (*self.next).set_next(id, data, path_id)
            } else {
                let layout: alloc::Layout = std::alloc::Layout::new::<Node<T>>();
                let temp_ptr: *mut u8 = std::alloc::alloc(layout);
                if temp_ptr.is_null() {std::alloc::handle_alloc_error(layout);}
                self.next = temp_ptr as *mut Node<T>;
                (*self.next).default(id, data, path_id);
            }
        }
    }

    fn set_side(&mut self, side: Side, id: u8) {
        self.get_node(id).side = side;
    }
    
    fn set_bridge(ptr_a: *mut Node<T>, ptr_b: *mut Node<T>, id_a: u8, id_b: u8) {
        //pass in heads and search for nodes
        unsafe{
            //lets store the desired addreses first then go ahead
            let temp_ptr_a: *mut Node<T>;
            let temp_ptr_b: *mut Node<T>;
            if !ptr_a.is_null() && !ptr_b.is_null() {
                temp_ptr_a = (*ptr_a).get_node(id_a);
                temp_ptr_b = (*ptr_b).get_node(id_b);
            } else {
                println!("One or more heads are empty.");
                return
            }


            match ptr_a.as_mut(){
                Some(node) => {node.get_node(id_a).bridged = temp_ptr_b;} //assigning to heads rather than finding the nodes
                None => {
                    println!("Head node is empty. Presumed Node ID: {}", id_a);
                    return
                }
            }
            match ptr_b.as_mut(){
                Some(node) => {node.get_node(id_b).bridged = temp_ptr_a;}
                None => {
                    println!("Head node is empty. Presumed Node ID: {}", id_b);
                    return
                }
            }
        }

    }



}


impl<T: std::fmt::Debug> Path<T> {
    fn new(id: u8) -> Self {
        return Path {id: id, start:  std::ptr::null_mut(), goal: None}
    }

    fn push(&mut self, node_id: u8, data: T) {
        unsafe {
            match self.start.as_mut() { //as_mut returns mut reference of caller
                Some(node) => {node.set_next(node_id, data, self.id)},
                None => {
                    //create layout
                    let layout: alloc::Layout = std::alloc::Layout::new::<Node<T>>(); //alt -> we can allocate before the caller and pass it in as parameters, store their address and then bridge based on these addresses
                    //allocate layout
                    let temp_ptr: *mut u8 = std::alloc::alloc(layout);
                    if temp_ptr.is_null() {std::alloc::handle_alloc_error(layout);} //error handle?
                    //assign to head of list
                    self.start = temp_ptr as *mut Node<T>;
                    //initiliaze can be created as a function to default all values
                    (*self.start).default(node_id, data, self.id);
                }
            }
        }
    }

    /*
    id_a should be the id needed to find the node on path_a to bridge to the node with id_b
    on path_b. 
    Essentially:
    path_a.bridge(id_a) = path_b.bridge
    path_b.bridge(id_b) = path_b.bridge
     */
    fn set_bridge(path_a: &Path<T>, path_b: &Path<T>, id_a: u8, id_b: u8) {
        Node::set_bridge(path_a.start, path_b.start, id_a, id_b); 
    }

    fn set_sides(path_a: &Path<T>, path_b: &Path<T>, id_a: u8, id_b: u8, side_a: Side, side_b: Side){
        if !(path_a.start.is_null()) && !(path_b.start.is_null()) {
            unsafe {
                (*path_a.start).set_side(side_a, id_a);
                (*path_b.start).set_side(side_b, id_b);    
            }
        } else {
            println!("One of heads are empty.");
        }
    }

    fn print_list(&self) {
        //check head and print it
        println!("\nPath_{} List:", self.id);
        let mut curr_ptr: &Node<T>;
        if !(self.start.is_null()) {
            curr_ptr = unsafe {&(*self.start)};
            println!("{:?}", curr_ptr); 
        } else {
            return 
        }

        while !curr_ptr.next.is_null() {
            curr_ptr = unsafe {&(*curr_ptr.next)};
            println!("{:?}", curr_ptr);
        }
        
    }

}

impl<T: std::fmt::Debug> Map<T> {
    pub fn new() -> Self {
        return Map {path_list: Vec::new(),}
    }

    pub fn print_contents(&self) {
        for elem in &self.path_list {
            elem.print_list();
        }
    }
}


//methods used to make structs coporeal

use rand::prelude::*;

use crate::pipes::math::pos_exponent;

pub fn spawn_map<T: std::fmt::Debug>(path_n: u8, bridge_n: u8) -> Map<i32> {
    //create map
    let mut map: Map<i32> = Map::new();

    //fill map with paths
    for _i in 0..path_n {
        map.path_list.push(Path::new(_i));
    }

    //create instance of an rng threaded object
    let mut rng: ThreadRng = rand::thread_rng();
    
    //populate nodes and create bridges
    for _i in 0..bridge_n {
        let mut rand: i8 = rng.gen_range(0..map.path_list.len()) as i8;//can be -1 or 1, but not 0    
        //store incrementation of first node_id
        let temp_node_id: i16 = (pos_exponent(_i as usize, 2) as i16) - ((pos_exponent(_i as usize, 2) as i16 ) - ((_i as i16) * 2));

        //check if first node is on the last or second path
        match rand {
            0 => {
                map.path_list[rand as usize].push(temp_node_id as u8, ((_i) as i32) * (-1));
                map.path_list[(rand as usize) + 1].push((temp_node_id as u8) + 1, ((_i) as i32) * (-1));
                Path::set_bridge(&map.path_list[rand as usize], &map.path_list[(rand as usize) + 1], temp_node_id as u8, (temp_node_id as u8) + 1);
                Path::set_sides(&map.path_list[rand as usize], &map.path_list[(rand as usize) + 1], temp_node_id as u8, (temp_node_id as u8) + 1, Side::L, Side::R); 
            },
            last if last == (map.path_list.len() - 1) as i8 => {
                map.path_list[rand as usize].push(temp_node_id as u8, ((_i) as i32) * (-1));
                map.path_list[(rand as usize) - 1].push((temp_node_id as u8) + 1, ((_i) as i32) * (-1));
                Path::set_bridge(&map.path_list[rand as usize], &map.path_list[(rand as usize) - 1], temp_node_id as u8, (temp_node_id as u8) + 1); 
                Path::set_sides(&map.path_list[rand as usize], &map.path_list[(rand as usize) - 1], temp_node_id as u8, (temp_node_id as u8) + 1, Side::R, Side::L); 
            },
            _ => {
                //store path pos to determine which path nodes will be pushed onto
                let temp_pos: u8 = rand as u8;
                map.path_list[temp_pos as usize].push(temp_node_id as u8, ((_i) as i32) * (-1));
                loop {
                    rand = rng.gen_range(-1..=1);
                    if rand == -1 || rand == 1 {
                        break
                    } else {
                        continue
                    }
                }
                map.path_list[((temp_pos as i8) + (rand)) as usize].push((temp_node_id as u8) + 1, ((_i) as i32) * (-1));
                Path::set_bridge(&map.path_list[temp_pos as usize], &map.path_list[((temp_pos as i8) + (rand)) as usize], temp_node_id as u8, (temp_node_id as u8) + 1);

                //i feel like there is a better way in writing this
                if rand == 1 {
                    Path::set_sides(&map.path_list[temp_pos as usize], &map.path_list[((temp_pos as i8) + (rand)) as usize], temp_node_id as u8, (temp_node_id as u8) + 1, Side::L, Side::R);                                                
                } else if rand == -1 {
                    Path::set_sides(&map.path_list[temp_pos as usize], &map.path_list[((temp_pos as i8) + (rand)) as usize], temp_node_id as u8, (temp_node_id as u8) + 1, Side::R, Side::L);
                }

                
            },
        }
    } 

    //set which will have the goal via rand
    let rand: usize = rng.gen_range(0..map.path_list.len());
    map.path_list[rand].push(255, -256);
    //set this last pushed node as the goal for rand path
    unsafe {map.path_list[rand].goal = Some((*map.path_list[rand].start).get_node(255));} 

    return map
}

//TODO: work on drawing it out MORE

pub fn generate_map_drawing<T: std::fmt::Debug>(map: &Map<T>, path_n: u8, bridge_n: u8) -> Vec<Vec<char>> {
    //cache pointers to lists
    let mut cached_lists: Vec<Option<&Node<T>>> = Vec::with_capacity(path_n as usize); 

    //fill cache
    for _i in 0..path_n as usize {
        cached_lists.push(unsafe {Some(&(*(map.path_list[_i].start)))});
    }
    
    //save last user/heart position
    let cols: usize = ((path_n * 4 ) - 3) as usize; 
    let rows: usize = ((bridge_n * 2) + 3) as usize;

    //initaliaze double vector
    let mut matrix: Vec<Vec<char>> = vec![vec!['_'; cols]; rows]; //vector inside a vector, it's [row[col]], but we will read it as [row][col] 
    //Ex. {{00, 01, 02}, {10, 11, 12}} \\each {} seperated by , is a given row

    //for every odd row, push only '|' ' '
    //every even row we check for _i or _j (Ex. 0 or 1 - increment from here) path that matches that id
    //when found push 'O' followed by three '-' and then another 'O' continue till colunms end
    for _i in 0..rows{
        match _i {
            0 => {
                //the initial row
                for _j in 0..cols {
                    if _j % 4 == 0 {
                        matrix[rows - _i - 1][_j] = 'X';
                    } else {
                        matrix[rows - _i - 1][_j] = ' ';  
                    }
                }
            },
            _i if (rows -_i - 1) == 0 => {
                //check for which path has the goal
                for _j in 0..cols {
                    match _j {
                        _j if _j % 4 == 0 => {
                            if map.path_list[_j / 4].goal.is_none() {
                                matrix[rows - _i - 1][_j] = ' ';
                            } else {
                                matrix[rows - _i - 1][_j] = '⋆';
                            }
                        },
                        _ => {
                            matrix[rows - _i - 1][_j] = ' ';
                        }
                    }
                }
            },

            _i if (rows - _i - 1) % 2 == 0 => {
                //check paths to find associated ids which will be incremented
                let mut _j: usize = 0;
                let mut found: bool = false;
                while _j < cols {
                    if _j % 4 == 0 && _j != cols - 1 && !found && !cached_lists[_j / 4].is_none(){   
                        
                        
                        let mut ptr: &Node<T> = cached_lists[_j / 4].unwrap(); //aleardy checked above

                        loop{
                            match ptr.id {
                                _id if _id == (_i as u8) - 1 || _id == (_i as u8 - 2) => {
                                    //the issue is _j continously changes with the third case
                                    unsafe {
                                        match ptr.bridged.as_ref() {
                                            Some(node) => {cached_lists[(_j / 4) + 1] = Some(&(*(node.next)))},
                                            None => {cached_lists[(_j / 4) + 1] = None;} 
                                        }
                                        match ptr.next.as_ref(){
                                            Some(node) => { cached_lists[_j / 4] = Some(&node);},
                                            None => {cached_lists[_j / 4] = None;}
                                        }
                                    }
                                    for _n in 0..4 {
                                        if _j % 4 != 0 {
                                            matrix[rows - _i - 1][_j] = '-';
                                        } else {    
                                            matrix[rows - _i - 1][_j] = 'O';
                                        }
                                        _j += 1;
                                    }
                                    if _j < cols {
                                        matrix[rows - _i - 1][_j] = 'O';
                                    }

                                    found = true;
                                    break;
                                },

                                _id if _id < (_i as u8) => {
                                    unsafe {
                                            match ptr.next.as_ref(){
                                            Some(node) => {ptr = node;},
                                            None => {break;},
                                        }
                                    }
                                },

                                _ => {
                                    break;
                                } 
                            }
                        }

                        if !found && _j < cols {
                            matrix[rows - _i - 1][_j] = '|';
                        }
              
                    } else if _j % 4 == 0{
                        matrix[rows - _i - 1][_j] = '|';
                    } else {
                        matrix[rows - _i - 1][_j] = ' ';
                    }
                    _j += 1;
                }
                // matrix[rows - _i - 1][_j];
                
            },
            _ => {
                //default '|' ' ' ' ' ' ' '|'
                for _j in 0..cols {
                    if _j % 4 == 0 {
                        matrix[rows - _i - 1][_j] = '|';
                    } else {
                        matrix[rows - _i - 1][_j] = ' ';
                    }
                }
            }
        }
    }
    
    return matrix
    
}

pub fn draw_map(matrix: &Vec<Vec<char>>, path_n: u8, bridge_n: u8) {
    let cols: usize = ((path_n * 4 ) - 3) as usize; 
    let rows: usize = ((bridge_n * 2) + 3) as usize;

    for _i in 0..rows{
        for _j in 0..cols {
            print!("{}", matrix[_i][_j]);
        }
        println!("");
    }
}

//TODO:: Work on creating user