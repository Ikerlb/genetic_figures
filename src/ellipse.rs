use rand::{Rng,rngs::ThreadRng};
use super::util::*;
use super::scanline::Scanline;
use cairo::Context;
use std::f64::consts::PI;

#[derive(Debug,Clone)]
pub struct Ellipse{
    x_limit:u32,
    y_limit:u32,
    x:u32,
    y:u32,
    xr:u32,
    yr:u32,
}

impl Ellipse{
    
    //TODO: test!!
    pub fn draw(&self,context:&Context){
        context.save();
        let x=self.x as f64;
        let y=self.y as f64;
        let xr=self.xr as f64;
        let yr=self.yr as f64;
        let width=xr*2.0;
        let height=yr*2.0;
        //context.translate((x+width)/2.0,(y+height)/2.0);
        context.translate(x,y);
        context.scale(xr,yr);
        context.arc(0.0,0.0,1.0,0.0,2.0*PI);
        context.restore();
    }

    pub fn crossover(&self,other:&Ellipse) -> Ellipse{
        Ellipse{
            x_limit:self.x_limit,
            y_limit:self.y_limit,
            x:self.x,
            y:other.y,
            xr:self.xr,
            yr:other.yr,
        }
    }
    
    pub fn mutate(&mut self,rand:&mut ThreadRng){
        let dx=rand.gen_range(-16i32,17i32);
        let dy=rand.gen_range(-16i32,17i32);
        match rand.gen_range(0,3) {
            0 => {
                self.x=clip_add(self.x,dx,0,self.x_limit-1);
                self.y=clip_add(self.y,dy,0,self.y_limit-1);
            },
            1 => self.xr=clip_add(self.xr,dx,2,self.x_limit-1),
            2 => self.yr=clip_add(self.yr,dy,2,self.y_limit-1),
            _ => unreachable!()
        }
    }

    //maybe do better? TEST!
    pub fn scanlines<'a>(&mut self,lines:&'a mut Vec<Scanline>) -> &'a[Scanline]{
        let c1=self.xr as f32/self.yr as f32;
        let c2=(self.yr*self.yr) as f32;
        let (mut ymin,mut ymax)=((self.y_limit-1) as i32,0u32);
        //TODO: ..=self.yr?
        let yr=self.yr as usize;
        for dj in 0..self.yr{
            let y1=self.y as i32 - dj as i32;
            let y2=self.y+dj;
            let x=((c2-(dj.pow(2) as f32)).sqrt()*c1) as i32;
            let mut x1=self.x as i32 - x;
            let mut x2=self.x + x as u32;
            if x1<0{
                x1=0;
            }
            if x2>=self.x_limit{
                x2=self.x_limit-1;
            }
            if y1>=0 && y1<self.y_limit as i32{
                //vec.push(y1 as usize,x1 as usize,x2));
                lines[y1 as usize].set(x1 as usize,x2 as usize,y1 as usize);
                if y1<ymin {
                    ymin=y1;
                }
            }
            if y2<self.y_limit && dj>0{
                //vec.push((y2,x1 as usize,x2));
                lines[y2 as usize].set(x1 as usize,x2 as usize,y2 as usize);
                if y2>ymax {
                    ymax=y2;
                }
            }
        }
        if ymin==(self.y_limit as i32)-1{
            ymin=0;
        }
        if ymax==0{
            ymax=self.y_limit-1;
        }
        &lines[(ymin as usize)..=(ymax as usize)]
    } 

    // fn scanlines_increment(&mut self){
    //
    // }

    pub fn new(x_limit:u32,y_limit:u32,rand:&mut ThreadRng) -> Ellipse{
        let x=rand.gen_range(0,x_limit);
        let xr=rand.gen_range(4,16);
        let y=rand.gen_range(0,y_limit);
        let yr=rand.gen_range(4,16);
        Ellipse{
            x_limit,
            y_limit,
            x,
            xr,
            y,
            yr,
       }
    }
}
