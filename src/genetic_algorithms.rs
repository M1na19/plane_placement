use crate::data_form::{Passanger,Group,Plane,Ocupation};
use crate::features::{get_random,get_random_permutation,to_number};
use crate::test::show_coridor;
use crate::{ROW_LENGHT,PLANE_ROWS};

const WAIT_ADD_ON_FAR_FROM_SEAT:i32=7;
const MOVE_DELAY:i32=2;


pub fn get_time_spent(plane:&mut Plane)->i32{

    let mut total_time:i32=0;

    //locurile in avion=>pentru verificare daca trebuie sa se ridice oamenii
    let mut  plane_matrix:Vec<Vec<Option<Passanger>>>=vec![vec![None;ROW_LENGHT];PLANE_ROWS+1];
    
    //passageri ce vor intra in avion
    let mut line:Vec<Passanger>=Vec::new();

    //adaugarea pasagerilor in coada
    for group in &plane.permutare_grupe{
        let random_perm=get_random_permutation(&plane.group[*group as usize-1].passangers);
        for passanger in  random_perm{
            line.push(passanger.clone());
        }
    }

    //contine locurile de pe coridor si in ce momente sunt ocupate
    let mut coridor_wait:Vec<Ocupation>=vec![Ocupation::new();PLANE_ROWS+1];

    for passanger in line{
        //pastreaza cand a ajuns pasagerul la randul curent
        let mut time_arrive_row:i32=0;

        for i in 0..coridor_wait.len(){

            //daca pasagerul a ajuns in locul sau
            if passanger.placement.0==i as i32{

                //tine cat trebuie sa astepte pasagerul ca alte persoane sa se ridice pentru ca el sa se aseze
                //asta daca este cazul
                let time_waiting_passangers={

                    //in ce parte a avionului sta 
                    let range={
                        if to_number(passanger.placement.1)<=ROW_LENGHT as i32/2{
                            to_number(passanger.placement.1) as usize..ROW_LENGHT/2-1
                        }
                        else{
                            ROW_LENGHT/2..to_number(passanger.placement.1) as usize
                        }
                    };
                    let mut count=0;
                    for i in range{
                        if plane_matrix[passanger.placement.0 as usize][i].is_some(){
                            count+=1;
                        }
                    }
                    count*WAIT_ADD_ON_FAR_FROM_SEAT
                };

                //se adauga perioada pe care pasagerul o consuma pentru a se pune la loc
                coridor_wait[i].period.push((time_arrive_row,time_arrive_row+passanger.time_ocuppied+time_waiting_passangers));

                //se verifica daca timpul pana pasagerul s-a pus este cel maxim
                total_time=std::cmp::max(total_time, time_arrive_row+passanger.time_ocuppied+time_waiting_passangers);

                //se tine faptul ca pasagerul a ajuns la locul sau
                plane_matrix[passanger.placement.0 as usize][to_number(passanger.placement.1) as usize-1]=Some(passanger);
                break;
            }

            //daca pasagerul nu e pe linia randului sau verificam daca poate merge in fata
            else{ 
                let end=coridor_wait[i+1].contains(time_arrive_row+MOVE_DELAY);
                match end {
                    
                    //in cazul acesta nu poate merge in fata deoarece o persoana deja ocupa locul
                    //se adauga timpul pe care il asteapta si cel dintre randuri
                    Some(period)=>{
                        coridor_wait[i].period.push(period);
                        time_arrive_row=period.1+MOVE_DELAY;
                    }

                    //altfel merge in fata normal adaugand timpul pe care il consuma mergand
                    None=>{
                        time_arrive_row+=MOVE_DELAY;
                    }
                }

            }   

        }
    }
    //show_coridor(&mut coridor_wait, total_time);
    total_time

}



use crate::{GENETIC_DIVERSITY,BEST_CHOSEN};

pub const UPPER_LIMIT_GROUPS:i32=10;
const MAX_CHANGE_IN_MUTATIONS:i32=20;

pub fn mutate(plane:Plane)->Vec<Plane>{
    let mutation_count:usize=GENETIC_DIVERSITY/BEST_CHOSEN;
    let group_permutation=mutation_count/3;

    //Daca nr de grupe nu depaseste limita
    let group_add:usize={
        if plane.group.len()<UPPER_LIMIT_GROUPS as usize{
            mutation_count/3
        }
        else{
            0
        }
    };
    let group_switch:usize=mutation_count-group_permutation-group_add;

    //Lista de mutati rezultate din avionul original
    let mut mutations:Vec<Plane>=Vec::new();
    for _ in 0..group_permutation{
        let mut new_plane=plane.clone();
    
        //se aleg grupe random schimbate
        let x=get_random(0, plane.permutare_grupe.len() as i32-1);
        let y=get_random(0, plane.permutare_grupe.len() as i32-1);
    
        //se face schimbul 
        new_plane.permutare_grupe.swap(x as usize, y as usize);
        mutations.push(new_plane);
    }
    for _ in 0..group_add{
        let mut new_plane=plane.clone();
        
        //se introduce un grup nou in avion
        new_plane.group.push(Group { id: plane.group.len() as i32+1, passangers: Vec::new()});
        new_plane.permutare_grupe.push(plane.group.len() as i32+1);
        //se determina un numar de pasageri furati de la fiecare grupa
        let passagers_stolen_count=((ROW_LENGHT*PLANE_ROWS) as f64/((plane.group.len()*(plane.group.len()+1)*plane.group.len()) as f64)).ceil() as usize;


        for i in 0..plane.group.len(){
            for __ in 0..passagers_stolen_count{
                //se alege un pasager random ce se extrage dintr-o grupa si e adaugat in cea noua
                let random_passanger_iter=get_random(0, new_plane.group[i].passangers.len() as i32-1) as usize;
                let passanger=new_plane.group[i].passangers.remove(random_passanger_iter);
                new_plane.group.last_mut().unwrap().passangers.push(passanger);
            }
        }
        mutations.push(new_plane);
    }
    for _ in 0..group_switch{

        // se determina un nr random de schimbari intre 0 si maxim
        let number_of_changes=get_random(0, MAX_CHANGE_IN_MUTATIONS);

        let mut new_plane=plane.clone();
        for __ in 0..number_of_changes{
            //se aleg doua grupe random
            let  group1=get_random(0, plane.group.len() as i32-1) as usize;
            let  group2=get_random(0, plane.group.len() as i32-1) as usize;

            //se aleg doi pasageri random din acion
            let x=get_random(0, new_plane.group[group1].passangers.len() as i32-1) as usize;
            let y=get_random(0, new_plane.group[group2].passangers.len() as i32-1) as usize;
            let passanger1=new_plane.group[group1].passangers[x].clone();
            let passanger2=new_plane.group[group2].passangers[y].clone();

            //interschimbarea
            {
                new_plane.group[group2].passangers[y]=passanger1;
                new_plane.group[group1].passangers[x]=passanger2;
            }
        }
        mutations.push(new_plane);
    }
    mutations
}
