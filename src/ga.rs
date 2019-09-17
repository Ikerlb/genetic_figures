use rand::{Rng,rngs::ThreadRng};
use super::rectangle::Rectangle;
use super::ellipse::Ellipse;
use super::figure::Figure;
use super::state::State;

//simple implementation of genetic algorithm, just to compare results vs stochastic hill climb
pub struct GeneticAlgorithm<'a>{
    n: usize,
    population: Vec<(Figure,i32)>,
    mutation_rate: i32,
    state: &'a mut State,
}


impl GeneticAlgorithm<'_>{
    pub fn new<'a>(state:&'a mut State,population_size:usize,mutation_rate:f64,mode:u32) -> GeneticAlgorithm<'a>{ 
        let (w,h)=state.dimensions;
        let mut p=Vec::with_capacity(population_size);
        for _ in 0..population_size{
            let mut f=Figure::new(mode,w,h,&mut state.rng);
            let cost=state.new_cost(&mut f);
            p.push((f,cost));
        }
        //sort worst to best. rank selection
        p.sort_by(|a,b| (b.1).cmp(&(a.1)));
        GeneticAlgorithm{
            n:population_size,
            population:p,
            mutation_rate:(mutation_rate*100.0) as i32,
            state:state,
        }
    }
   
    pub fn get_best_fitness(&self) -> (Figure,i32){
        self.population.last().unwrap().clone()
    }
    
    //TODO: is it worth it taking diversity into account? 
    //TODO: also phenotype diversity vs genotype diversity.
    pub fn next_generation(&mut self){
        let n=self.n;
        let mut new_population:Vec<(Figure,i32)>=Vec::with_capacity(n);
        let limit=((n*(n-1))/2)+1;
        for _ in 0..n{
            let rn1=self.state.rng.gen_range(0, limit as i32) as f32;
            let idx1=0.5+(0.25+(2.0*rn1)).sqrt().ceil();
            let p1=&self.population[idx1 as usize-1].0;
            let rn2=self.state.rng.gen_range(0, limit as i32) as f32;
            let idx2=0.5+(0.25+(2.0*rn2)).sqrt().ceil();
            let p2=&self.population[idx2 as usize-1].0;

            //let nf=(&mut self.population[idx1].0).crossover(&mut self.population[idx2].0);
            let mut nf=p1.crossover(p2);

            //println!("p1: {:?}\np2: {:?}\nson: {:?}",p1,p2,nf);

            if self.state.rng.gen_range(0,100)<self.mutation_rate*100{
                nf.mutate(&mut self.state.rng);
            }
            let cost=self.state.new_cost(&mut nf);
            new_population.push((nf,cost));
        }
        new_population.sort_by(|a,b| (b.1).cmp(&(a.1)));
        //for (f,c) in new_population.iter(){
        //    println!("{:?},{:?}",f,c);
        //}
        self.population=new_population;
    }
}
