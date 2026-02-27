use byond_fn::byond_fn;
use rayon_ca::CellularAutomaton;
use serde_json;

#[byond_fn]
pub fn create_and_process(x:usize, y:usize, steps:u64, alive_probability:Option<usize>) -> String
{
    let mut c = CellularAutomaton::new(x, y);
    c.randomize_prob(alive_probability.unwrap_or(50) as f64 / 100.0).steps(steps);
    match serde_json::to_string(&c.cells.clone()){
        Ok(s) => s,
        Err(e) => format!("ERR: {}", e)
    }
}

