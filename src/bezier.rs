//quadratic bezier curves
use rand::{Rng,rngs::ThreadRng};
use super::util::*;
use super::scanline::Scanline;
use cairo::Context;
use super::img::Color;

#[derive(Debug,Clone)]
pub struct Bezier{
    x_limit:u32,
    y_limit:u32,
    x1:u32,
    y1:u32,
    cx:u32,
    cy:u32,
    x2:u32,
    y2:u32,
}

impl Bezier{
    pub fn draw(&self,context:&Context,color:&Color){
        context.set_line_width(1.0);
        let (x1,y1,cx,cy,x2,y2)=self.unpack_f64();
        context.move_to(x1,y1);
        context.curve_to(cx,cy,cx,cy,x2,y2);
        let (r,g,b,a)=color.normalize();
        context.set_source_rgba(r,g,b,a);
        context.stroke();
    }

    pub fn crossover(&self,other:&Bezier) -> Bezier{
        Bezier{
            x_limit:self.x_limit,
            y_limit:self.y_limit,
            x1:self.x1,
            y1:self.y1,
            cx:self.cx,
            cy:other.cy,
            x2:other.x2,
            y2:other.y2,
        }
    }

    fn unpack(&self) -> (u32,u32,u32,u32,u32,u32){
          (self.x1,self.y1,self.cx,self.cy,self.x2,self.y2)
    }

    fn unpack_f64(&self) -> (f64,f64,f64,f64,f64,f64){
        (self.x1 as f64,self.y1 as f64,self.cx as f64,self.cy as f64,self.x2 as f64,self.y2 as f64)
    }

    pub fn mutate(&mut self,rand:&mut ThreadRng){
        let dx=rand.gen_range(-16i32,17i32);
        let dy=rand.gen_range(-16i32,17i32);
        match rand.gen_range(0,3){
            0 => {
                self.x1=clip_add(self.x1,dx,0,self.x_limit-1);
                self.y1=clip_add(self.y1,dy,0,self.y_limit-1);
            },
            1 => {
                self.cx=clip_add(self.cx,dx,0,self.x_limit-1);
                self.cy=clip_add(self.cy,dy,0,self.y_limit-1);
            }, 
            2 => {
                self.x2=clip_add(self.x2,dx,0,self.x_limit-1);
                self.y2=clip_add(self.y2,dy,0,self.y_limit-1);
            },
            _ => unreachable!(),
        }
    }

    //quadratic_bezier(startx:usize,starty:usize,endx:usize,endy:usize,controlx:usize,controly:usize,t:f32)
    pub fn scanlines(&mut self) -> Vec<Scanline>{
        let mut sl:Vec<Scanline>=Vec::new();
        let (x1,y1,cx,cy,x2,y2)=self.unpack();
        let steps=20;
        let step=1.0/(steps as f32);
        let mut t=step;
        let mut prevx=x1;
        let mut prevy=y1;
        for _ in 0..steps{
            let (nextx,nexty)=quadratic_bezier(x1,y1,x2,y2,cx,cy,t);
            sl.append(&mut points_to_scanlines(&bresenham(prevx,prevy,nextx,nexty)));
            prevx=nextx;
            prevy=nexty;
            t+=step;
        }
        sl
        /*let mut xstart=points[0].0;
        let mut prevx=points[0].0;
        let mut prevy=points[0].1;
        println!("{:?}",points);
        for (x,y) in points {
            if y!=prevy{
                sl.push(Scanline::new(min(prevx,xstart),max(prevx,xstart),prevy));
                xstart=prevx;
            }
            prevy=y;
            prevx=x;
        }
        sl.push(Scanline::new(min(prevx,xstart),max(prevx,xstart),prevy));
        sl*/
    }


    pub fn new(x_limit:u32,y_limit:u32,rand:&mut ThreadRng) -> Bezier{
        let x1=rand.gen_range(0,x_limit);
        let mut x2=rand.gen_range(0,x_limit);
        let y1=rand.gen_range(0,y_limit);
        let mut y2=rand.gen_range(0,y_limit);
        while x1==x2&&y1==y2{
            y2=rand.gen_range(0,y_limit);
            x2=rand.gen_range(0,x_limit);
        }
        let cx=rand.gen_range(0,x_limit);
        let cy=rand.gen_range(0,y_limit);
        Bezier{
            x_limit,
            y_limit,
            x1,
            y1,
            cx,
            cy,
            x2,
            y2,
        }
    }
}
