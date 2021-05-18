#[derive(Debug,Clone)]
pub struct Scanline{
    pub y:usize,
    pub x1:usize,
    pub x2:usize,
}

impl Scanline{
    pub fn new(x1:usize,x2:usize,y:usize) -> Scanline{
        Scanline{
            x1,x2,y
        }
    }
   
    pub fn unpack(&self) -> (usize,usize,usize){
        (self.y,self.x1,self.x2)
    }


    /*pub fn empty() -> Scanline{
        Scanline{
            x1:0,
            x2:0,
            y:0
        }
    }*/

    /*pub fn set(&mut self,x1:usize,x2:usize,y:usize){
        self.y=y;
        self.x1=x1;
        self.x2=x2;
    }*/
}   
