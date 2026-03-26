use std::time::{SystemTime};
use std::{collections::HashMap, sync::Mutex};
use std::sync::{Arc, LazyLock};

use byond_fn::byond_fn;
use rayon_ca::CellularAutomaton;
use rayon_ca::strats::{conway_next, corridors_next};
use serde_json;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");
#[byond_fn] pub fn get_version() -> String { VERSION.to_string() }

pub static LAST_TIME: LazyLock<SystemTime> = LazyLock::new(|| SystemTime::now());
#[byond_fn] pub fn get_time() -> String {
	LAST_TIME.elapsed().unwrap().as_nanos().to_string()
}

pub static AUTOMATONS:LazyLock<Mutex<
		HashMap<String, Arc<Mutex<CellularAutomaton>>>
	>> = LazyLock::new(|| Mutex::new(HashMap::new()));

type Rule = fn(&CellularAutomaton, usize, usize) -> i32;

pub static STRATS: LazyLock<HashMap<&'static str, Rule>> = LazyLock::new(|| {
	let mut m:HashMap<&'static str, Rule> = HashMap::new();
	m.insert("conway", conway_next);
	m.insert("corridors", corridors_next);
	m
});

#[byond_fn]
pub fn create_and_process(
	x:usize,
	y:usize,
	steps:u64,
	alive_probability:Option<usize>,
	strat_id:Option<String>
) -> String {
	let strat:Rule;
	match strat_id {
		Some(id) => match STRATS.get(id.as_str()).copied() {
    		Some(r) => strat = r,
    		None => {
      			return "ERR: STRAT NOT FOUND".to_string()
      		},
		},
		None => strat = conway_next as Rule,
	};
	let mut c = CellularAutomaton::new_with_processor(
		x,
		y,
		strat
	);
	c.randomize_prob(alive_probability.unwrap_or(50) as f64 / 100.0).steps(steps);
	match serde_json::to_string(&c.cells){
		Ok(s) => s,
		Err(e) => format!("ERR: {}", e)
	}
}


#[byond_fn]
pub fn create_automaton(id: String, x: Option<usize>, y: Option<usize>, strat_id:Option<String>) -> String {
	let mut at = AUTOMATONS.lock().unwrap();
	if at.contains_key(&id) {
		return "ERR: EXISTS".to_string()
	}
	let mut c = CellularAutomaton::new(x.unwrap_or(2), y.unwrap_or(2));
	if let Some(id) = strat_id {
		match STRATS.get(id.as_str()).copied() {
    		Some(r) => c.set_processor(r),
    		None => return format!("ERR: Strat '{}' not found", id)
		};
	}
	at.insert(id, Arc::new(Mutex::new(c)));
	"OK".to_string()
}
#[byond_fn]
pub fn set_strat(id: String, strat_id: String) -> String {
	let mut map = AUTOMATONS.lock().unwrap();
	if let Some(c) = map.get_mut(&id){
		return match c.lock(){
			Ok(mut c) => {
				match STRATS.get(strat_id.as_str()){
					Some(s) => c.set_processor(*s),
					None => return format!("ERR: Strat '{}' not found", strat_id)
				};
				"OK".to_string()
			},
			Err(e) => format!("ERR: {}", e)
		}
	}
	format!("ERR: Automaton '{}' not found", id)
}

#[byond_fn]
pub fn set_xy(id: String, x: usize, y: usize) -> String {
	let mut map = AUTOMATONS.lock().unwrap();
	if let Some(c) = map.get_mut(&id){
		return match c.lock(){
			Ok(mut c) => {
				c.set_xy(x, y, 0);
				"OK".to_string()
			},
			Err(e) => format!("ERR: {}", e)
		}
	}
	format!("ERR: Automaton '{}' not found", id)
}

#[byond_fn]
pub fn randomize(id:String, alive_prob:Option<usize>) -> String {
	let mut map = AUTOMATONS.lock().unwrap();
	if let Some(c) = map.get_mut(&id){
		return match c.lock(){
			Ok(mut c) => {
				c.randomize_prob(alive_prob.unwrap_or(50) as f64 / 100.0);
				"OK".to_string()
			},
			Err(e) => format!("ERR: {}", e)
		}
	}
	format!("ERR: Automaton '{}' not found", id)
}

#[byond_fn]
pub fn make_steps(id:String, steps:u64) -> String{
	let automaton = {
		let map = AUTOMATONS.lock().unwrap();
		map.get(&id).cloned()
	};
	if let Some(c) = automaton {
		c.lock().unwrap().steps(steps);
		return "OK".to_string();
	}
	format!("ERR: Automaton '{}' not found", id)
}

pub fn request_automaton(id: &String) -> Option<Arc<Mutex<CellularAutomaton>>> {
	let map = AUTOMATONS.lock().unwrap();
	return match map.get(id){
		Some(c) => Some(c.clone()),
		None => None,
	}
}

#[byond_fn]
pub fn get_field_json(id: String) -> String {
	if let Some(c) = request_automaton(&id) {
		return match serde_json::to_string(&c.lock().unwrap().cells){
			Ok(s) => s,
			Err(e) => format!("ERR: {}", e)
		}
	}
	format!("ERR: Automaton '{}' not found", id)
}

#[byond_fn]
pub fn get_field_raw(id: String) -> String {
	match request_automaton(&id) {
		Some(c) =>
			return match &c.lock(){
				Ok(ca) => {
					let mut dot = String::new();
					let len = ca.cells.len();
					for (idx, row) in ca.cells.iter().enumerate() {
						for (jdx, v) in row.iter().enumerate() {
						    if jdx > 0 {
						        dot.push(' ');
						    }
						    dot.push_str(&v.to_string());
						}
					    if idx < len - 1 {
							dot.push(';');
					    }
					}
					dot
				},
				Err(e) => format!("ERR: {}", e)
			},
		None => format!("ERR: Automaton '{}' not found", id),
	}
}
