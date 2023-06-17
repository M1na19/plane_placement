use crate::{PLANE_ROWS,ROW_LENGHT};
use crate::genetic_algorithms::get_time_spent;
use crate::data_form::{Passanger,Plane,Ocupation};
use crate::features::{to_letter};

pub fn back_front()->Plane{
    let mut plane=Plane::intialize(3);
    plane.permutare_grupe=vec![3,2,1];
    let x=1;
    for i in 0..3{
        plane.group[i as usize].id=i+1;
    }
    
    for i in 1..=PLANE_ROWS{
        for j in 1..=ROW_LENGHT{
            let mut passanger:Passanger=Passanger::new(0, (i as i32,to_letter(j as i32)));
            if i<=PLANE_ROWS/4{
                passanger.group=1;
            }
            else if i<=PLANE_ROWS/2{
                passanger.group=2;
            }
            else {
                passanger.group=3;
            }

            plane.group[passanger.group as usize-1].passangers.push(passanger);
        }
    }
    plane
    
}
pub fn best_config()->Plane{
    let mut plane=Plane::intialize(6);
    let x=1;
    plane.permutare_grupe=vec![1,6,2,5,3,4];
    for i in 0..3{
        plane.group[i as usize].id=i+1;
    }
    
    for i in 1..=PLANE_ROWS{
        for j in 1..=ROW_LENGHT{
            let mut passanger:Passanger=Passanger::new(0, (i as i32,to_letter(j as i32)));
            
            if i%2==0 && j<=ROW_LENGHT/2{
                passanger.group=1;
            }
            else if i%2==0{
                passanger.group=2;
            }
            else if i%2==1 && j<=ROW_LENGHT/2{
                passanger.group=3;
            }
            else{
                passanger.group=4;
            }

            plane.group[passanger.group as usize-1].passangers.push(passanger);
        }
    }
    plane
}

use crate::GET_GENERAL_IDEA;
pub fn test(plane:&mut Plane)->i32{
    let mut result=0;
    for _ in 0..GET_GENERAL_IDEA{
        result+=get_time_spent(plane);
    }
    result/=GET_GENERAL_IDEA;
    result
}

pub fn show_coridor(coridor:&mut Vec<Ocupation>,end:i32){
    for moment in 0..=end{
        for rand in 1..=PLANE_ROWS{
            if coridor[rand].contains(moment).is_some(){
                print!("*");
            }
            else{
                print!("[]");
            }
        }
        println!();
    }
}