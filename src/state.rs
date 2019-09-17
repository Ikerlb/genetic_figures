use image::{RgbaImage,Rgba};
use super::{imgutil,figure::Figure,minimize,ga::GeneticAlgorithm};

use rand::rngs::ThreadRng;

#[derive(Debug)]
pub struct State{
    pub current: RgbaImage,
    target: RgbaImage,
    alpha: u8,
    pub dimensions: (u32,u32),
    pub cost:i32,
    pub rng:ThreadRng,
}

impl State{
    pub fn new(target:RgbaImage,alpha:u8)->State{
        let dimensions=target.dimensions();
        let (w,h)=dimensions;
        let (r,g,b)=imgutil::average_color(&target);
        let current=RgbaImage::from_fn(w,h,|_,_|{Rgba([r,g,b,255])});
        let cost=imgutil::full_cost(&target,&current);
        let rng=rand::thread_rng();
        let st= State{
            current,
            target,
            alpha,
            dimensions,
            cost,
            rng,
        };

        return st;
    }

    

    pub fn step(&mut self,generations:u32,population:usize,mode:u32){
        let (w,h)=self.dimensions;
        //self.cost=nc
        let mut ga=GeneticAlgorithm::new(self,population,0.01,mode);
        let gbf=ga.get_best_fitness();
        let mut bf=gbf.0;
        let mut bc=gbf.1;
        for _ in 0..generations{
            ga.next_generation();
            let (figure,c)=ga.get_best_fitness();
            if c<bc {
                bc=c;
                bf=figure;
            }
        }
        let (mut bf,bc)=minimize::hill_descent(self,100,bf);
        let color=imgutil::compute_color(&self.target,&self.current,&bf.scanlines(),self.alpha);
        imgutil::composite(&mut self.current,&bf.scanlines(),&color);
        self.cost=bc;
    }

    pub fn new_cost(&mut self,figure:&mut Figure) -> i32{
        let color=imgutil::compute_color(&self.target,&self.current,&figure.scanlines(),self.alpha);
        imgutil::partial_cost(&self.target,&mut self.current,self.cost,&figure.scanlines(),&color)
    }

    fn best_random_figure(&mut self,repeat:u32,mode:u32)->Figure{
        let (w,h)=self.dimensions;
        let mut bc=self.cost;
        let mut bf=Figure::new(mode,w,h,&mut self.rng);
        for _ in 0..repeat{
            let mut f=Figure::new(mode,w,h,&mut self.rng);
            let color=imgutil::compute_color(&self.target,&self.current,&f.scanlines(),self.alpha);
            let c=imgutil::partial_cost(&self.target,&mut self.current,self.cost,&f.scanlines(),&color);
            if c<bc{
                bc=c;
                bf=f.clone();
            }
        }
        bf
    }
}
