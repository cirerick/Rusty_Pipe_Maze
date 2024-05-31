use crate::pipes::Node;


/*
we do not need to import anything


*/

struct User<T> {
    pos: *mut Node<T>,
}

impl<T> User<T>{
    fn new(pos: *mut Node<T>) -> Self {
        return User {pos: pos}
    }
    
    
    fn next(&mut self) {
        unsafe {
            if !((*(self.pos)).next.is_null()) {
                self.pos = (*(self.pos)).next;
            } else if (*(self.pos)).id == 255 {
                println!("WINNER");
            } else {
                println!("LOSER");
            }
        }
    }

    fn cross(&mut self) {
        unsafe {
            if !(((*self.pos)).bridged.is_null()) {
                self.pos = (*(self.pos)).next;
            } else if (*(self.pos)).id == 255 {
                println!("WINNER");
            } else {
                println!("LOSER");
            } 
        }
    } 
}