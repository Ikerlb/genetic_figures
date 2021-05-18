use rand::rngs::ThreadRng;
use super::rectangle::Rectangle;
use super::ellipse::Ellipse;
use super::line::Line;
use super::bezier::Bezier;
use super::scanline::Scanline;
use cairo::Context;
use super::img::Color;


#[derive(Debug,Clone)]
pub enum Figure{
    Rectangle(Rectangle),
    Ellipse(Ellipse),
    Line(Line),
    Bezier(Bezier),
}

impl Figure{
    
    pub fn mutate(&mut self,rand:&mut ThreadRng){
        match self{
            Figure::Rectangle(rect)  => rect.mutate(rand),
            Figure::Ellipse(ellipse) => ellipse.mutate(rand),
            Figure::Line(line)       => line.mutate(rand),
            Figure::Bezier(bezier)   => bezier.mutate(rand),
        };
    }

    pub fn scanlines(&mut self) -> Vec<Scanline>{
        match self{
            Figure::Rectangle(rect)  => rect.scanlines(),
            Figure::Ellipse(ellipse) => ellipse.scanlines(),
            Figure::Line(line)       => line.scanlines(),
            Figure::Bezier(bezier)   => bezier.scanlines(),
        }
    }

    pub fn draw(&self,context:&Context,color:&Color){
        match self{
            Figure::Rectangle(rect)  => rect.draw(&context,&color),
            Figure::Ellipse(ellipse) => ellipse.draw(&context,&color),
            Figure::Line(line)       => line.draw(&context,&color),
            Figure::Bezier(bezier)   => bezier.draw(&context,&color),
        }
    }

    pub fn crossover(&self,other:&Figure) -> Figure{
        match self{
            Figure::Rectangle(r) => match other{
                                    Figure::Rectangle(r2) => Figure::Rectangle(r.crossover(r2)),
                                    _                     => panic!("Can't crossover two different figures"),
                                  },
            Figure::Ellipse(e)  => match other{
                                    Figure::Ellipse(e2)   => Figure::Ellipse(e.crossover(e2)),
                                    _                     => panic!("Can't crossover two different figures"),
                                  },
            Figure::Line(l)    => match other{
                                    Figure::Line(l2)      => Figure::Line(l.crossover(l2)),
                                    _                     => panic!("Can't crossover two different figures"), 
                                  },
            Figure::Bezier(b)  => match other{
                                    Figure::Bezier(b2)    => Figure::Bezier(b.crossover(b2)),
                                    _                     => panic!("Can't crossover two different figures"),
                                  },
        }
    }


    pub fn new(mode:u32,w:u32,h:u32,rand:&mut ThreadRng)->Figure{
        match mode{
            1=>Figure::Rectangle(Rectangle::new(w,h,rand)),
            2=>Figure::Ellipse(Ellipse::new(w,h,rand)),
            3=>Figure::Line(Line::new(w,h,rand)),
            4=>Figure::Bezier(Bezier::new(w,h,rand)),
            _=>panic!("Unimplemented mode"),
        }
    }
}
