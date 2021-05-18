use rand::{Rng,rngs::ThreadRng};
use super::util::*;
use super::scanline::Scanline;
use cairo::Context;
use super::img::Color;

#[derive(Debug,Clone)]
pub struct Line{
    x_limit:u32,
    y_limit:u32,
    x1:u32,
    y1:u32,
    x2:u32,
    y2:u32,
}

impl Line{
    pub fn draw(&self,context:&Context,color:&Color){
        context.set_line_width(1.0);
        let (x1,y1,x2,y2)=self.unpack();
        context.move_to(x1 as f64,y1 as f64);
        context.line_to(x2 as f64,y2 as f64);
        let (r,g,b,a)=color.normalize();
        context.set_source_rgba(r,g,b,a);
        context.stroke();
    }

    pub fn crossover(&self,other:&Line) -> Line{
        Line{
            x_limit:self.x_limit,
            y_limit:self.y_limit,
            x1:self.x1,
            y1:self.y1,
            x2:other.x2,
            y2:other.y2,
        }
    }

    fn unpack(&self) -> (u32,u32,u32,u32){
        /*if self.x2<self.x1{
            let aux=self.x1;
            self.x1=self.x2;
            self.x2=aux;
        }
        if self.y2<self.y1{
            let aux=self.y1;
            self.y1=self.y2;
            self.y2=aux;
        }*/
        (self.x1,self.y1,self.x2,self.y2)
    }

    pub fn mutate(&mut self,rand:&mut ThreadRng){
        let (x1,y1,x2,y2)=self.unpack();
        let dx=rand.gen_range(-16i32,17i32);
        let dy=rand.gen_range(-16i32,17i32);
        if rand::random(){
            self.x1=clip_add(x1,dx,0,self.x_limit-1);
            self.y1=clip_add(y1,dy,0,self.y_limit-1);
        }
        else{
            self.x2=clip_add(x2,dx,0,self.x_limit-1);
            self.y1=clip_add(y2,dy,0,self.y_limit-1);
        }
    }

    //TODO: Is it safe to say there are no 0 px lines?
    pub fn scanlines(&mut self) -> Vec<Scanline>{
        let (x1,y1,x2,y2)=self.unpack();
        let bsh=bresenham(x1,y1,x2,y2);
        /*let mut sl:Vec<Scanline>=Vec::new();       
        let mut xstart=bsh[0].0;
        let mut prevx=bsh[0].0;
        let mut prevy=bsh[0].1;
        for (x,y) in bsh {
            if y!=prevy{
                sl.push(Scanline::new(min(prevx,xstart),max(prevx,xstart),prevy));    
                xstart=prevx;
            }
            prevy=y;
            prevx=x;
        }
        sl.push(Scanline::new(min(prevx,xstart),max(prevx,xstart),prevy));
        //println!("{:?}",self);
        //println!("{:?}",sl);
        sl*/
        points_to_scanlines(&bsh)
    }

    pub fn new(x_limit:u32,y_limit:u32,rand:&mut ThreadRng) -> Line{
        let x1=rand.gen_range(0,x_limit);
        let mut x2=rand.gen_range(0,x_limit);
        while x1==x2{
            x2=rand.gen_range(0,x_limit);    
        }
        let y1=rand.gen_range(0,y_limit);
        let mut y2=rand.gen_range(0,y_limit);
        while y1==y2{
            y2=rand.gen_range(0,y_limit);
        }
        Line{
            x_limit,
            y_limit,
            x1,
            y1,
            x2,
            y2,
        }
    }
}
