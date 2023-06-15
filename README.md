# **Algoritm genetic ce analizeaza cea mai buna organizare a urcarii in avion**
## Algoritm genetic
>***Algoritmele genetice*** functioneaza similar cu evolutia speciilor, incepand de la anumite valori sau anumite gene, cele care aduc rezultatele cele mai bune '*supravietuiesc*', generatie dupa generatie genele bune se propaga iar cele proaste mor.\
>>Cele mai bune gene sufera mutati astfel incat valorile sa poata fi schimbate pe parcursul generatiilor, dar sa pastreze majoritatea particularitatilor bune a indivizilor bune  
## Genele algoritmului
#### ***Genele*** acestui algoritm sau variabilele ce se schimba si se perfectioneaza de-a lungul generatiilor sunt:
- **Numarul grupelor**
>Numarul grupelor determina cat de sectionati sunt pasagerii ce intra in avion, seamana cu ***first-class***, ***second-class*** si ***economy***
>>Aceste grupe intra in acelasi timp, dar pasagerii dintr-o grupa intra intr-o ordine aleatoare
- **Permutarea grupelor**
>Permutarea grupelor determina ordinea in care intra grupele
>>Spre exemplu daca exista 3 grupe: 1,2,3 cele 3 grupe pot intra sub forma 1-2-3,1-3-2,2-1-3,2-3-1,3-1-2,3-2-1
- **Continutul grupelor**
>Poate cea mai importanta gena este organizarea pasagerilor in grupe
>>Ex:\
>>1 poate contine 10 pasageri (1A,4C,7F,9B,13A,14G...=>acestea sunt pozitii in avion)
## **Cum functioneaza?**
#### Organizarea datelor
1. **Pasagerii**
>*Pasagerii* sunt definiti de:
>>- ***Grupa*** este definita drept un numar ce reprezinta indicele categoriei in care pasagerul se incadreaza
>>- ***Plasamentul*** e definit de ***rand*** si ***scaunul*** pe care se afla
>>>**Planul avionului**\
>>>1A 1B 1C | | 1D 1E 1F\
>>>2A 2B 2C | | 2D 2E 2F\
>>>3A 3B 3C | | 3D 3E 3F\
>>>......................................
>>- ***Timpul ocupat*** este un numar de secunde consumate pana pasagerul isi pune bagajele, scoate ce are nevoie si se pune pe scaun si variaza de la pasager la pasager
```rs
#[derive(Debug,Clone,Copy)]
struct Passanger{
    group:i32,
    placement:(i32,char),
    time_ocuppied:i32
}
```
2. **Grupa**
>*Grupurile* sunt definite de:
>>- ***Id*** 
>>- ***Sir de pasageri*** pe care grupa le contine
```rs
#[derive(Debug,Clone)]
struct Group{
    id:i32,
    passangers:Vec<Passanger>
}
```
3. **Avion**
>*Avioanele* sunt definte de:
>>- ***Sir de grupuri***
>>- ***Permutarea grupelor*** (explicata mai devreme)
>>- ***Rezultatul***=> cat dureaza ca toti pasagerii sa ajunga in locul lor
```rs
struct Plane{
    group:Vec<Group>,
    permutare_grupe:Vec<i32>,
    result:i32
}
```
### Functiile structurilor
1. **Crearea unui pasager nou**
>*Pasagerii* sunt creati stiind grupul in care se afla si pozitia pe care o are, timpul pe care un pasager il consuma este ales *random*
```rs
Passanger::new(group:i32,placement:(i32,char))
```
2. **Crearea unui grup nou**
>*Grupurile* sunt create fara informatie aditionala sub forma sa **default**
>>Default:\
>>```id=0```\
>>```pasangeri=lista de pasageri cu 0 elemente```
```rs
let group=Group::new();
println!("{:?}",group);
```
>Output
```bash
{id:0, passangers: []}
```
3. **Initializarea avionului**
>*Avionul* este creat stiind nr de grupe pe care le contine sub forma sa **default**
>>Default:\
>>```group=sir de grupuri de tip default```\
>>```permutare_grupe=o permutare random a sirulul 1,2,3..nr_grupe```\
>>```result=0```
```rs
Plane::initialise(nr_groups:i32)
```
4. **Configurarea avionului**
```rs
plane.add_config(nr_groups);
```
>Avionul, Grupele si Pasageri primesc valori random
```rs
fn add_config(&mut self,nr_groups:i32){
        let mut passanger_per_group=vec![ROW_LENGHT*PLANE_ROWS/nr_groups as usize;nr_groups as usize];
        passanger_per_group[get_random(0, nr_groups-1) as usize]+=ROW_LENGHT*PLANE_ROWS%nr_groups as usize;
```
>In acest fragment este declarat un vector ***passanger_per_group*** ce tine cati pasageri ar trebui sa aiba fiecare grupa pentru a le echilibra
```rs
        for i in 0..nr_groups{
            self.group[i as usize].id=i+1;
        }
```
>Se pun id-urile in structurile **Group**
```rs
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
```
>Trecand prin toate scaunele avionului sunt puse in grupuri random
5. **Afisarea avionului**
```rs
plane.show();
```
>*Avionul* este afisat sub forma:\
x y z | | a b c\
a x b | | a b x\
........................\
Result:......\
>Unde a,b,c,x,y,z,etc.. sunt indexele unor grupuri
>>Ex:
```bash
 1 1 2 | | 3 2 2
 2 2 1 | | 3 1 3
 4 1 2 | | 2 1 4
 2 3 1 | | 4 1 1

Result:114 sec
```
## Constante ale programului
>Constantele au fost alese dupa un avion mai mic
- **Numarul de randuri**
```rs
const PLANE_ROWS:usize=20;
```
- **Numarul de scaune pe un rand**
```rs
const ROW_LENGHT:usize=6;
```
- **Maximul si Minimul pe care un pasager poate sa-l consume inainte sa se aseze**
```rs
const WAIT_TIME_MAX:i32=15;
const WAIT_TIME_MIN:i32=5;
```
- **Timpul care se adauga cand alti pasageri trebuie sa se ridice pentru a te aseza**
```rs
const WAIT_ADD_ON_FAR_FROM_SEAT:i32=15;
```
- **Timpul consumat pentru a ajunge la urmatorul rand**
```rs
const MOVE_DELAY:i32=2;
```
- **Numarul maxim de grupuri**
```rs
const UPPER_LIMIT_GROUPS:i32=10;
```
- **Numarul maxim de schimbari intr-o mutatie**
```rs
const MAX_CHANGE_IN_MUTATIONS:i32=20;
```
## Functii ajutatoare
- **get_random(limita la stanga, limita la dreapta)**
>Returneaza un nr in intervalul dat
```rs
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
```
- **get_random_permutation(vector de elemente)**
>Returneaza o permutare a vectorului dat
```rs
fn get_random_permutation<T:Clone>(original:&Vec<T>)->Vec<T>{
    let lenght=original.len();
    let mut permutation:Vec<T>=original.clone();
    for i in 0..lenght{
        permutation.swap(i, get_random(0, lenght as i32-1) as usize);
    }
    permutation
}
```
- **to_letter(number) si to_number(letter)**
> Transforma din char in numar
>>Ex:\
>> 'A'<=>1\
>> 'B'<=>2\
>>etc..
```rs
fn to_letter(num:i32)->char{
    char::from_u32(u32::try_from('A').unwrap()+num as u32-1).expect("Cannot convert i32 to letter")
}


fn to_number(letter:char)->i32{
    (u32::try_from(letter).unwrap()-u32::try_from('A').unwrap()) as i32+1
}
```