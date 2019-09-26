use super::state::State;
use super::figure::Figure;


pub fn hill_descent(state:&mut State,age:usize,figure:&Figure) -> (Figure,i32){
    let mut bc=state.cost;
    let mut figure=figure.clone();
    for _ in 0..age{
        let prev=figure.clone();
        figure.mutate(&mut state.rng);
        let c=state.new_cost(&mut figure);
        if c>=bc{
            figure=prev;
        } else{
            bc=c;
        }
    }
    (figure,bc)
}
