use crate::pipes;

use crate::pipes::Node;


/*
we do not need to import anything


*/
#[derive(Debug)]
pub struct User<T> {
    pub pipe_pos: *mut Node<T>,
    pub ascii_pos: (usize, usize),
}

impl<T> User<T>{
    pub fn new(path: &pipes::Path<T>, start_pos_x: usize, start_pos_y: usize) -> Self {
        return User {pipe_pos: path.start,
                     ascii_pos: (start_pos_x, start_pos_y),        
                }
    }
    
    
    pub fn next(&mut self, running: &mut bool) {
        unsafe {
            if !((*(self.pipe_pos)).next.is_null()) {
                self.pipe_pos = (*(self.pipe_pos)).next;
            } else if (*(self.pipe_pos)).id == 255 {
                println!("WON");
                *running = false;
            } else {
                println!("LOSS");
                *running = false;
            }
        }
    }

    pub fn cross(&mut self, running: &mut bool) {
        unsafe {
            if !(((*self.pipe_pos)).bridged.is_null()) {
                self.pipe_pos = (*(self.pipe_pos)).bridged;
            } 
        }
    } 
}