use crate::features::{get_random,get_random_permutation,to_letter};
use crate::{ROW_LENGHT,PLANE_ROWS};

const WAIT_TIME_MAX:i32=10;//seconds
const WAIT_TIME_MIN:i32=5;

#[derive(Debug,Clone,Copy)]
pub struct Passanger{
    pub group:i32,
    pub placement:(i32,char),
    pub time_ocuppied:i32
}

impl Passanger{
    pub fn new(group:i32,placement:(i32,char))->Self{
        Self{
            group:group,
            placement:placement,
            time_ocuppied:get_random(WAIT_TIME_MIN, WAIT_TIME_MAX)
        }
    }
}




#[derive(Debug,Clone)]
pub struct Group{
    pub id:i32,
    pub passangers:Vec<Passanger>
}

impl Group{
    pub fn new()->Self {
        Self { id: 0, passangers: Vec::new() }
    }
}




#[derive(Debug,Clone)]
pub struct Plane{
    pub group:Vec<Group>,
    pub permutare_grupe:Vec<i32>,
    pub result:i32
}

impl Plane{
    pub fn intialize(nr_groups:i32)->Self{
        Self{
            group:vec![Group::new();nr_groups as usize],
            permutare_grupe:get_random_permutation(&(1..=nr_groups).collect()),
            result:0
        }
        
    }
    pub fn add_config(&mut self,nr_groups:i32){
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
    pub fn show(&self){
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
pub struct Ocupation{
    pub period:Vec<(i32,i32)>
}
impl Ocupation{
    pub fn new()->Self{
        Ocupation { period: Vec::new() }
    }
    pub fn contains(&mut self,moment:i32)->Option<(i32,i32)>{
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