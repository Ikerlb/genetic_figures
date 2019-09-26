use rand::{Rng,rngs::ThreadRng};
use super::util::*;
use super::scanline::Scanline;
use cairo::Context;
use std::cmp;

#[derive(Debug,Clone)]
pub struct Rectangle{
    x_limit:u32,
    y_limit:u32,
    x1:u32,
    y1:u32,
    x2:u32,
    y2:u32,
}

impl Rectangle{
    
    pub fn draw(&self,context:&Context){
        let x1=cmp::min(self.x1,self.x2) as f64;
        let x2=cmp::max(self.x1,self.x2) as f64;
        let y1=cmp::min(self.y1,self.y2) as f64;
        let y2=cmp::max(self.y1,self.y2) as f64;
        context.rectangle(x1,y1,x2-x1,y2-y1);
    }


    pub fn crossover(&self,other:&Rectangle) -> Rectangle{
        Rectangle{
            x_limit:self.x_limit,
            y_limit:self.y_limit,
            x1:self.x1,
            y1:other.y1,
            x2:self.x2,
            y2:other.y2
        }
    }
    
    pub fn mutate(&mut self,rand:&mut ThreadRng){
        let (x1,y1,x2,y2)=self.bounds();
        let dx=rand.gen_range(-16i32,17i32);
        let dy=rand.gen_range(-16i32,17i32);
        if rand::random() {
            self.x1=clip_add(x1,dx,0,self.x_limit-1);
            self.y1=clip_add(y1,dy,0,self.y_limit-1);
        } else{
            self.x2=clip_add(x2,dx,0,self.x_limit-1);
            self.y2=clip_add(y2,dy,0,self.y_limit-1);
        }
    }

    pub fn scanlines<'a>(&mut self,lines:&'a mut Vec<Scanline>) -> &'a[Scanline]{
        let (x1,y1,x2,y2)=self.bounds_as_usize();
        for j in y1..=y2{
            lines[j].set(x1,x2,j);
        }
        &lines[y1..=y2]
    }

    

    // fn set_figure(&mut self,other:&Self){
    //     self.x1=other.x1;
    //     self.x2=other.x2;
    //     self.y1=other.y1;
    //     self.y2=other.y2;
    // }


    //TODO: check, as this allows initialization of 0-area rects
    pub fn new(x_limit:u32,y_limit:u32,rand:&mut ThreadRng) -> Rectangle{
        let x1=rand.gen_range(0,x_limit);
        let x2=clip(x1+rand.gen_range(1,33),0,x_limit-1);
        let y1=rand.gen_range(0,y_limit);
        let y2=clip(y1+rand.gen_range(1,33),0,y_limit-1);

        Rectangle{
            x_limit,
            y_limit,
            x1,
            x2,
            y1,
            y2,
        }
    }

    fn bounds(&mut self) -> (u32,u32,u32,u32){
        if self.x2<self.x1{
            let aux=self.x1;
            self.x1=self.x2;
            self.x2=aux;
        }
        if self.y2<self.y1{
            let aux=self.y1;
            self.y1=self.y2;
            self.y2=aux;
        }
        (self.x1,self.y1,self.x2,self.y2)
    }

    fn bounds_as_usize(&mut self) -> (usize,usize,usize,usize){
        let (x1,y1,x2,y2)=self.bounds();
        (x1 as usize,y1 as usize,x2 as usize,y2 as usize)
    }
}
