use image::RgbaImage;
use super::{figure::Figure,minimize,ga::GeneticAlgorithm};
use super::img::Img;
use cairo::{ImageSurface,Context,Format};

use rand::rngs::ThreadRng;

pub struct State{
    pub current: Img,
    target: Img,
    alpha: u8,
    pub dimensions: (u32,u32),
    pub cost:i32,
    pub rng:ThreadRng,
    pub surface:ImageSurface,
    context:Context,
    pub counter:u32,
}

impl State{
    pub fn new(target_rgba:RgbaImage,alpha:u8,output_width:i32,output_height:i32)->State{
        let surface=ImageSurface::create(Format::ARgb32, output_width,output_height).unwrap();
        let context=Context::new(&surface);
        let dimensions=target_rgba.dimensions();
        let (w,h)=dimensions;
        let target=Img::from_rgba_image(&target_rgba);
        let color=target.average_color(255);
        let (r,g,b,a)=color.normalize();
        context.scale(output_width as f64/w as f64,output_height as f64/h as f64);
        context.set_source_rgba(r,g,b,a);
        context.paint();
        let current=Img::from_fn(|_,_| color.unpack());
        let cost=current.full_cost(&target);
        let rng=rand::thread_rng();
        let counter=0;
        let st= State{
            current,
            target,
            alpha,
            dimensions,
            cost,
            rng,
            context,
            surface,
            counter,
        };
        return st;
    }

    

    pub fn step(&mut self,generations:u32,population:usize,mode:u32,sweep:bool) -> u32{
        //self.cost=nc
        let mut ga=GeneticAlgorithm::new(self,population,0.1,mode);
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
        if sweep{
            let (bfa,bca)=minimize::hill_descent(self,500,&bf);
            bf=bfa;
            bc=bca;
        }
        let sl=bf.scanlines();
        let color=self.current.compute_color(&self.target,&sl,self.alpha);
        //draw in current
        self.current.composite(&sl,&color);
        //draw in context
        bf.draw(&(self.context),&color);
        //set cost
        self.cost=bc;
        let c=self.counter;
        self.counter=0;
        c
    }

    pub fn new_cost(&mut self,figure:&mut Figure) -> i32{
        self.counter+=1;
        //let color=imgutil::compute_color(&self.target,&self.current,&figure.scanlines(),self.alpha);
        let scanlines=figure.scanlines();
        let color=self.current.compute_color(&self.target,&scanlines,self.alpha);
        self.current.partial_cost(&self.target,self.cost,&scanlines,&color)
        //imgutil::partial_cost(&self.target,&mut self.current,self.cost,&figure.scanlines(),&color)
    }

    //fn best_random_figure(&mut self,repeat:u32,mode:u32)->Figure{
    //    let (w,h)=self.dimensions;
    //    let mut bc=self.cost;
    //    let mut bf=Figure::new(mode,w,h,&mut self.rng);
    //    for _ in 0..repeat{
    //        let mut f=Figure::new(mode,w,h,&mut self.rng);
    //        let color=imgutil::compute_color(&self.target,&self.current,&f.scanlines(),self.alpha);
    //        let c=imgutil::partial_cost(&self.target,&mut self.current,self.cost,&f.scanlines(),&color);
    //        if c<bc{
    //            bc=c;
    //            bf=f.clone();
    //        }
    //    }
    //    bf
    //}
}
