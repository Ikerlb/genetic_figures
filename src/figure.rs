use rand::rngs::ThreadRng;
use super::rectangle::Rectangle;
use super::ellipse::Ellipse;
use super::scanline::Scanline;
use cairo::Context;

#[derive(Debug,Clone)]
pub enum Figure{
    Rectangle(Rectangle),
    Ellipse(Ellipse),
}

impl Figure{
    
    pub fn mutate(&mut self,rand:&mut ThreadRng){
        match self{
            Figure::Rectangle(rect) => rect.mutate(rand),
            Figure::Ellipse(ellipse) => ellipse.mutate(rand),
        };
    }

    pub fn scanlines<'a>(&mut self,lines:&'a mut Vec<Scanline>) -> &'a[Scanline]{
        match self{
            Figure::Rectangle(rect) => rect.scanlines(lines),
            Figure::Ellipse(ellipse) => ellipse.scanlines(lines),
        }
    }

    pub fn draw(&self,context:&Context){
        match self{
            Figure::Rectangle(rect) => rect.draw(&context),
            Figure::Ellipse(ellipse) => ellipse.draw(&context),
        }
    }

    pub fn crossover(&self,other:&Figure) -> Figure{
        return match self{
            Figure::Rectangle(rect) => match other{
                                            Figure::Rectangle(rect2) => Figure::Rectangle(rect.crossover(rect2)),
                                            _ => panic!("Can't crossover two different figures"),
                                        },
            Figure::Ellipse(ellipse) => match other{
                                            Figure::Ellipse(ellipse2) => Figure::Ellipse(ellipse.crossover(ellipse2)),
                                            _ => panic!("Can't crossover two different figures"),
                                        },
        };
    }


    pub fn new(mode:u32,w:u32,h:u32,rand:&mut ThreadRng)->Figure{
        match mode{
            1=>Figure::Rectangle(Rectangle::new(w,h,rand)),
            2=>Figure::Ellipse(Ellipse::new(w,h,rand)),
            _=>panic!("Unimplemented mode"),
        }
    }
}
