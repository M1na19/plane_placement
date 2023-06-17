use rand::Rng;

use crate::data_form::Plane;
use crate::BEST_CHOSEN;


pub fn get_random(limit_st:i32,limit_dr:i32)->i32{
    let mut rng=rand::thread_rng();
    if limit_st==limit_dr{
        return limit_dr;
    }
    else if limit_st>limit_dr {
        panic!("Invalid");
    }
    rng.gen_range(limit_st..=limit_dr)
}
pub fn get_random_permutation<T:Clone>(original:&Vec<T>)->Vec<T>{
    let lenght=original.len();
    let mut permutation:Vec<T>=original.clone();
    for i in 0..lenght{
        permutation.swap(i, get_random(0, lenght as i32-1) as usize);
    }
    permutation

}




pub fn to_letter(num:i32)->char{
    char::from_u32(u32::try_from('A').unwrap()+num as u32-1).expect("Cannot convert i32 to letter")
}
pub fn to_number(letter:char)->i32{
    (u32::try_from(letter).unwrap()-u32::try_from('A').unwrap()) as i32+1
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





pub fn get_top_performers(results:&Vec<Plane>)->Vec<Plane>{
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