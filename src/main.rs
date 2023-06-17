use std::env;

mod features;
mod data_form;
mod test;
mod genetic_algorithms;

use features::{get_random,get_top_performers};
use data_form::Plane;
use test::{test,back_front,best_config};
use genetic_algorithms::{get_time_spent,mutate,UPPER_LIMIT_GROUPS};
//Folosind un algoritm genetic vedem cate grupe e cel mai optim si cate ar trebui sa contina
//Imbarcarea lor cum e cel mai optim=> in ce ordine
//Un grup nu trb sa aiba toti membri apropriati
//In cazul unui rezultat random fac modificati


const PLANE_ROWS:usize=30;
const ROW_LENGHT:usize=6;//asta inseamna 120 de locuri










const GENETIC_DIVERSITY:usize=1000;
const BEST_CHOSEN:usize=100;
const GET_GENERAL_IDEA:i32=10;
fn genetic_algorithm(){
    let NUMBER_OF_GENERATIONS:i32={
        match env::var("generations"){
           Ok(nr)=>{
               str::parse::<i32>(nr.as_str()).unwrap()
           }
           Err(error)=>{
               println!("{}",error);
               100
           }
        }
       };
       println!("Number of generations: {}",NUMBER_OF_GENERATIONS);
     //Indivizii primei generatii
     let mut planes_scope=vec![Plane::intialize(get_random(1, UPPER_LIMIT_GROUPS));GENETIC_DIVERSITY];

     //configurarea primei generatii
     for plane in &mut planes_scope{
         plane.add_config(plane.group.len() as i32);
     }
 
 
     for _ in 0..NUMBER_OF_GENERATIONS{
 
         //calcularea rezultatelor 
         for mut plane in &mut planes_scope{
             plane.result={
                let mut total=0;
                for _ in 0..GET_GENERAL_IDEA{
                    total+=get_time_spent(plane);        
                }
                total/=GET_GENERAL_IDEA;
                total
            }
         }
 
         //pastrarea elementelor cu cele mai bune rezultate
         let top:Vec<Plane>=get_top_performers(&planes_scope);
         planes_scope=Vec::new();
         for top_plane in top{
             planes_scope.append(&mut mutate(top_plane))
         }
     }
 
     //formarea ultimei generatii
     for mut plane in &mut planes_scope{
        plane.result={
            let mut total=0;
            for _ in 0..GET_GENERAL_IDEA{
                total+=get_time_spent(plane);        
            }
            total/=GET_GENERAL_IDEA;
            total
        }       
     }
     let top:Vec<Plane>=get_top_performers(&planes_scope);
 
     let mut best=top[0].clone();
     //printarea celei mai bune configuratii
     println!("{:?}",best);
     top[0].show();
     println!("Permuatarea Grupelor:{:?}",best.permutare_grupe);
     println!("Result:{}",best.result);
     for _ in 0..10{
         println!("{}",test(&mut best));
     }
}
fn main() {
    genetic_algorithm();
}
