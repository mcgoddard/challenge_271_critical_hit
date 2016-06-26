use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 3 {
    	let sides = &args[1];
    	let hitpoints = &args[2];
    	let sides: u32 = match sides.trim().parse()
	    {
	    	Ok(num) => num,
	    	Err(_) => panic!("Sides was not a positive number!"),
	    };
	    let hitpoints: u32 = match hitpoints.trim().parse()
	    {
	    	Ok(num) => num,
	    	Err(_) => panic!("Hitpoints was not a positive number!"),
	    };
    	println!("Chance to kill: {}", recursive_find(sides, hitpoints));
    }
    else {
        println!("Incorrect number of arguments supplied.");
    }
}

fn recursive_find(d: u32, h: u32) -> f64 {
	if h <= d {
		let df = d as f64;
		let hf = h as f64;
		return ((df + (1 as f64)) - hf) / df;
	}
	else {
		let df = d as f64;
	    return recursive_find(d, h - d) / df;
	}
}