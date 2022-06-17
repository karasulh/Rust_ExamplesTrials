//This codes are find the barycenter of some masses by parallelizing, so it is faster than previos version.
//add a dependency for itertools and rayon

extern crate rayon;//for parallelism
extern crate itertools;//extra iteration methods
use rayon::prelude::*;

mod bodies;
use bodies::get_values;

#[derive(Debug,Clone,Copy)]
pub struct Body{
    x:f64,
    y:f64,
    z:f64,
    mass:f64
}

fn average(a:f64, b:f64) -> f64 {
    (a+b)/2.0
}

fn average_with_mass(a:f64, b:f64, amass:f64, bmass: f64) -> f64{
    average(a*amass,b*bmass) / (amass + bmass)
}

fn merge_two_bodies(a:Body, b:Body) -> Body {
    Body{
        x:average_with_mass(a.x, b.x, a.mass, b.mass),
        y:average_with_mass(a.y, b.y, a.mass, b.mass),
        z:average_with_mass(a.z, b.z, a.mass, b.mass),
        mass: a.mass + b.mass
    }
}

fn merge_all_bodies_recursive(bodies: &[Body]) -> Body { //use slices/array
    println1!("Bodies: {}",bodies.len());

    if bodies.len() == 1{
        return bodies[0];
    } 

    let tuples: Vec<(Body,Body)> = bodies.iter().tuples().collect(); //Vec<_> is also OK.
    let mut merged_bodies: Vec<_> = tuples.into_par_iter().map(|(a,b)| merge_two_bodies(*a,*b)).collect(); //into_par_item: parallelized iteration
    
    if bodies.len() % 2 != 0 { //if one item is left when the number of item is odd
        merged_bodies.push(bodies[bodies.len() - 1]);
    }

    return merge_all_bodies_recursive(&merged_bodies); // continue for recursion

}

fn main(){

    let bodies = get_values();
    let barycenter = merge_all_bodies_recursive(&bodies);
    println!("Barycenter: ({}, {}, {}),\nMass: {}",barycenter.x,barycenter.y,barycenter.z,barycenter.mass);

}