#include<random>
#include<stdlib.h>
#include<vector>
const int MAX_GROUP_TO_SEATS_RATIO=20;
//try implemanting template for list<> because u need it at Passanger List and Period List
int min(int a,int b){
    if(a<b){
        return a;
    }else{
        return b;
    }
}
int max(int a,int b){
    if(a>b){
        return a;
    }else{
        return b;
    }
}

//Mutations can be:
//Steal from one group to other

struct Passanger{
    int groupID;
    int row;
    char seat;
    Passanger* next;
};
struct Group{
    Passanger* first=NULL,*last=NULL;
    int nrPassangers;
    void AddPassanger(Passanger* p){
        if(first==NULL ){
            first=p;
        }else if(last==NULL){
            last=p;
            first->next=last;
        }else{
            last->next=p;
            last=p;
        }
    }
    //If u want to kill it use free after
    Passanger* RemovePassangerAfter(Passanger* p){
        if(p==last){
            return NULL;
        }else if(p->next==last){
            Passanger* rem=last;
            last=p;
            p->next=NULL;
            return rem;
        }else{
            Passanger* rem=p->next;
            p->next=p->next->next;
            return rem;
        }
    }
};
struct Airplane{
    Group* groups;
    int nrGroups;
    int nrRows;
    int nrSeats;
    void Copy(Airplane* a){
        //copy the standard stuff
        nrGroups=a->nrGroups;
        nrRows=a->nrRows;
        nrSeats=a->nrSeats;
        groups=(Group*)calloc(nrGroups,sizeof(Group));
        for(int i=0;i<nrGroups;i++){
            Passanger*current=a->groups[i].first;
            for(int j=0;j<a->groups[i].nrPassangers;j++){
                Passanger passangerCopy;

                //copy all the details
                passangerCopy.groupID=i;
                passangerCopy.row=current->row;
                passangerCopy.seat=current->seat;

                //give the copy away
                groups[i].AddPassanger(&passangerCopy);
                current=current->next;
            }
        }
    }
    //Dealocate Memory 
    void Kill(){
        free(groups);
    }
    //Mutation degree should be a number between 1 and 100
    //Remember to free mutations
    Airplane* Mutate(int maxMutations,int mutationDegree,int nrResults){
        srand(NULL);

        Airplane* mutations=(Airplane*)calloc(nrResults,sizeof(Airplane));
        for(int i=0;i<nrResults;i++){
            int nrMutations=rand()%maxMutations+maxMutations*mutationDegree/100;
            for(int j=0;j<nrMutations;j++){
                mutations[i].Copy(this);

                //find random groups
                int randGroup1=rand()%nrGroups+1;
                while(mutations[i].groups[randGroup1].nrPassangers==0){
                    randGroup1=rand()%nrGroups+1;
                }
                int randGroup2=rand()%nrGroups+1;
                while(randGroup2==randGroup1){
                    randGroup2=rand()%nrGroups+1;
                }

                //find random passanger in first Group
                int randPassanger=rand()%mutations[i].groups[randGroup1].nrPassangers+1;

                //get the passanger and give it to group2 
                Passanger* current=mutations[i].groups[randGroup1].first;
                for(int k=1;k<randPassanger;k++){
                    current=current->next;
                }
                Passanger* stolen=mutations[i].groups[randGroup1].RemovePassangerAfter(current);
                mutations[i].groups[randGroup2].AddPassanger(stolen);
            }
        }
        return mutations;
    }
    int Performance(){
        struct Interval{
            struct Period{
                int start,end;
                bool Intersects(Period* p){
                    if(start<p->end && start>p->start){
                        return true;
                    }else if(end>p->start && end<p->end){
                        return true;
                    }else{
                        return false;
                    }
                }
            }*periods;
            
            Period* Reunion(Period* i,Period* j){
                if(i->Intersects(j)==false){
                    return NULL;
                }else{
                    Period* z;
                    z->start=min(i->start,j->start);
                    z->end=max(i->end,j->end);
                    return z;
                }
            }
        }*Occupancy;
    }
};

Airplane* GenerateRandomAirplane(int nrRows,int nrSeats){
    Airplane a;
    srand(NULL);
    a.nrGroups=rand()%(nrRows*nrSeats/MAX_GROUP_TO_SEATS_RATIO)+1;
    a.groups=(Group*)calloc(a.nrGroups,sizeof(Group));

    for(int i=0;i<nrRows;i++){
        for(int j=0;j<nrSeats;j++){
            //find random group
            int randomGroup=rand()%a.nrGroups+1;

            //give it the details
            Passanger* current=new Passanger();
            current->row=i+1;
            current->seat=char('A'+j);
            current->groupID=randomGroup;
            
            //give it away
            a.groups[randomGroup].AddPassanger(current);
            a.groups[randomGroup].nrPassangers++;
        }
    }
}
