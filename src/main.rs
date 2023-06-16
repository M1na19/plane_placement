use std::mem::swap;

//Folosind un algoritm genetic vedem cate grupe e cel mai optim si cate ar trebui sa contina
//Imbarcarea lor cum e cel mai optim=> in ce ordine
//Un grup nu trb sa aiba toti membri apropriati
//In cazul unui rezultat random fac modificati
use rand::Rng;

//maybe make number of groups limited 

const PLANE_ROWS:usize=20;
const UPPER_LIMIT_GROUPS:i32=10;
const ROW_LENGHT:usize=6;//asta inseamna 120 de locuri
const WAIT_TIME_MAX:i32=15;//seconds
const WAIT_TIME_MIN:i32=5;
const WAIT_ADD_ON_FAR_FROM_SEAT:i32=15;//sunt marimi aproximative le putem modifica           
const MOVE_DELAY:i32=2;
const MAX_CHANGE_IN_MUTATIONS:i32=20;

fn get_random(limit_st:i32,limit_dr:i32)->i32{
    let mut rng=rand::thread_rng();
    if limit_st==limit_dr{
        return limit_dr;
    }
    else if limit_st>limit_dr {
        panic!("Invalid");
    }
    rng.gen_range(limit_st..=limit_dr)
}
fn get_random_permutation<T:Clone>(original:&Vec<T>)->Vec<T>{
    let lenght=original.len();
    let mut permutation:Vec<T>=original.clone();
    for i in 0..lenght{
        permutation.swap(i, get_random(0, lenght as i32-1) as usize);
    }
    permutation

}
fn to_letter(num:i32)->char{
    char::from_u32(u32::try_from('A').unwrap()+num as u32-1).expect("Cannot convert i32 to letter")
}
fn to_number(letter:char)->i32{
    (u32::try_from(letter).unwrap()-u32::try_from('A').unwrap()) as i32+1
}



#[derive(Debug,Clone,Copy)]
struct Passanger{
    group:i32,
    placement:(i32,char),
    time_ocuppied:i32
}
impl Passanger{
    fn new(group:i32,placement:(i32,char))->Self{
        Self{
            group:group,
            placement:placement,
            time_ocuppied:get_random(WAIT_TIME_MIN, WAIT_TIME_MAX)
        }
    }
}
#[derive(Debug,Clone)]
struct Group{
    id:i32,
    passangers:Vec<Passanger>
}

impl Group{
    fn new()->Self {
        Self { id: 0, passangers: Vec::new() }
    }
}
#[derive(Debug,Clone)]
struct Plane{
    group:Vec<Group>,
    permutare_grupe:Vec<i32>,
    result:i32
}

impl Plane{
    fn intialize(nr_groups:i32)->Self{
        Self{
            group:vec![Group::new();nr_groups as usize],
            permutare_grupe:get_random_permutation(&(1..=nr_groups).collect()),
            result:0
        }
        
    }
    fn add_config(&mut self,nr_groups:i32){
        let mut passanger_per_group=vec![ROW_LENGHT*PLANE_ROWS/nr_groups as usize;nr_groups as usize];
        passanger_per_group[get_random(0, nr_groups-1) as usize]+=ROW_LENGHT*PLANE_ROWS%nr_groups as usize;
        // Vectorul passanger_per_group tine cati pasageri ar trebui sa aiba fiecare grupa pentru a le echilibra


        for i in 0..nr_groups{
            self.group[i as usize].id=i+1;
        }
        // Se pun id-urile grupelor

        //Se parcurg toate locurile din avion
        for i in 1..=PLANE_ROWS{
            for j in 1..=ROW_LENGHT{
                let mut random_group=get_random(1, nr_groups);
                while passanger_per_group[random_group as usize-1]==0{
                    random_group=get_random(1, nr_groups);
                }
                passanger_per_group[random_group as usize-1]-=1;
                let pasanger=Passanger::new(random_group, (i as i32,to_letter(j as i32)));
                self.group[pasanger.group as usize-1].passangers.push(pasanger);
            }
        }
    }
    fn find(&self,placement:(i32,char))->Passanger{
        for group in &self.group{
            for passanger in &group.passangers{
                if passanger.placement==placement{
                    return passanger.clone();
                }
            }
        }
        todo!()
    }
    fn show(&self){
        for i in 1..=PLANE_ROWS{
            for j in 1..=ROW_LENGHT/2{
                print!("{} ",self.find((i as i32,to_letter(j as i32))).group);
            }
            print!(" ");
            for j in ROW_LENGHT/2+1..=ROW_LENGHT{
                print!("{} ",self.find((i as i32,to_letter(j as i32))).group);
            }
            println!();
        }
    }
}
#[derive(Clone)]
struct Ocupation{
    period:Vec<(i32,i32)>
}
impl Ocupation{
    fn new()->Self{
        Ocupation { period: Vec::new() }
    }
    fn contains(&mut self,moment:i32)->Option<(i32,i32)>{
        let mut end:Option<(i32,i32)>=None;
        for per in &mut self.period{
            if moment>=per.0 && moment<=per.1{
                end=Some(per.clone());
                return end;
            }
        }
        return end;
        
    }
}
fn get_time_spent(plane:&mut Plane)->i32{

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
    total_time

}


fn mutate(plane:Plane)->Vec<Plane>{
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


fn binary_search(mut range:(usize,usize),v:&Vec<Plane>,my_result:i32)->usize{
    if range.0<=range.1{
        let mid=(range.0+range.1)/2;
        if v[mid].result<my_result{
            range.0=mid+1;
        }
        else if mid>0{
            range.1=mid-1;
        }
        else{
            return 0;
        }
        binary_search(range, v, my_result)
    }
    else{
        range.0
    }
}
fn get_top_performers(results:&Vec<Plane>)->Vec<Plane>{
    let mut top:Vec<Plane>=Vec::new();
    top.push(results[0].clone());
    while top.len()<BEST_CHOSEN{
        let position=binary_search((0,top.len()-1), &top,results[top.len()].result);
        if position<top.len(){
            top.insert(position, results[top.len()].clone());
        }
        else{
            top.push(results[top.len()].clone());
        }
    }


// Pana cand vectorul nou se umple pana la lungimea de BEST_CHOSEN
// valorile din vectorul initial sunt impinse la final daca au cel mai prost rezultat.
// Altfel sunt inserate in pozitia determinata de cautarea binara


    for i in top.len()..results.len(){
        top.insert(binary_search((0,top.len()-1), &top, results[i].result), results[i].clone());
        top.split_off(BEST_CHOSEN).clear();
    }
    top
}










const GENETIC_DIVERSITY:usize=1000;
const BEST_CHOSEN:usize=100;
const NUMBER_OF_GENERATIONS:i32=10000;

fn main() {
    //Indivizii primei generatii
    let mut planes_scope=vec![Plane::intialize(get_random(1, UPPER_LIMIT_GROUPS));GENETIC_DIVERSITY];

    //configurarea primei generatii
    for plane in &mut planes_scope{
        plane.add_config(plane.group.len() as i32);
    }


    for _ in 0..NUMBER_OF_GENERATIONS{

        //calcularea rezultatelor 
        for mut plane in &mut planes_scope{
            plane.result=get_time_spent(plane);        
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
        plane.result=get_time_spent(plane);        
    }
    let top:Vec<Plane>=get_top_performers(&planes_scope);

    let mut best=top[0].clone();
    //printarea celei mai bune configuratii
    println!("{:?}",best);
    top[0].show();
    println!("Permuatarea Grupelor:{:?}",best.permutare_grupe);
    println!("Result:{}",best.result);
    for i in 0..10{
        println!("{}",test(&mut best));
    }
}
fn back_front()->Plane{
    let mut plane=Plane::intialize(3);
    let x=1;
    plane.permutare_grupe=vec![1,6,2,5,3,4];
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

            plane.group[passanger.group as usize].passangers.push(passanger);
        }
    }
    plane
    
}
fn best_config()->Plane{
    let mut plane=Plane::intialize(6);
    let x=1;

    for i in 0..3{
        plane.group[i as usize].id=i+1;
    }
    
    for i in 1..=PLANE_ROWS{
        for j in 1..=ROW_LENGHT{
            let mut passanger:Passanger=Passanger::new(0, (i as i32,to_letter(j as i32)));
            if (i%2==0 && j==1) || (i%2==1 && j==ROW_LENGHT){
                passanger.group=1;
            }
            else if (i%2==0 && j==2) || (i%2==1 && j==ROW_LENGHT-1){
                passanger.group=2;
            }
            else if (i%2==0 && j==3) || (i%2==1 && j==ROW_LENGHT-2){
                passanger.group=3;
            }
            else if (i%2==1 && j==1) || (i%2==0 && j==ROW_LENGHT){
                passanger.group=4;
            }
            else if (i%2==1 && j==2) || (i%2==0 && j==ROW_LENGHT-1){
                passanger.group=5;
            }
            else if (i%2==1 && j==3) || (i%2==0 && j==ROW_LENGHT-2){
                passanger.group=6;
            }

            plane.group[passanger.group as usize-1].passangers.push(passanger);
        }
    }
    plane
}
fn test(plane:&mut Plane)->i32{
    return get_time_spent(plane);
}