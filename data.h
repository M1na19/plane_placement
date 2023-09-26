#include<random>
#include<stdlib.h>
#include<template.h>
#include<iostream>
const int MAX_GROUP_TO_SEATS_RATIO=20;
const int ROW_TIME=2;//sec
const int MAX_WAIT_TIME=10;
const int MIN_WAIT_TIME=2;

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

struct Passanger{
    int groupID;
    int row;
    int waitTime;
    char seat;
};
bool sortByRowSeat(Passanger &x,Passanger &y){
    if(x.row==y.row)return x.seat<y.seat;
    return x.row<y.row;
}
struct Group{
    smartstore<Passanger> passangers;
};
struct Airplane{
    smartstore<Group> groups;
    int nrRows;
    int nrSeats;
    void Copy(Airplane& a){
        nrRows=a.nrRows;
        nrSeats=a.nrSeats;
        groups=a.groups.copy();
    }

    //Mutations can be steals of passangers from one place to another or switches between the order groups enter
    //Mutation degree should be a number between 1 and 100
    smartstore<Airplane>& Mutate(int maxMutations,int mutationDegree,int nrResults){
        srand(NULL);

        smartstore<Airplane> mutations;
        for(int i=0;i<nrResults;i++){
            int nrMutations=rand()%maxMutations+maxMutations*mutationDegree/100;
            Airplane a;
            for(int j=0;j<nrMutations;j++){
                a.Copy(*this);

                //find random groups
                int randGroup1=rand()%groups.size()+1;
                while(a.groups[randGroup1].passangers.size()==0){
                    randGroup1=rand()%groups.size()+1;
                }
                int randGroup2=rand()%groups.size()+1;
                while(randGroup2==randGroup1){
                    randGroup2=rand()%groups.size()+1;
                }

                //find random passanger in first Group
                int randPassanger=rand()%a.groups[randGroup1].passangers.size()+1;

                //get the passanger and give it to group2 
                Passanger removed=a.groups[randGroup1].passangers.cutout(randPassanger);
                a.groups[randGroup2].passangers.pushBack(removed);
            }
            //switch random groups
            //find random groups
            int randGroup1=rand()%groups.size()+1;
            int randGroup2=rand()%groups.size()+1;
            a.groups.swap(randGroup1,randGroup2);

            mutations.pushBack(a);
        }
        return mutations;
    }
    int Performance(){
        struct Interval{
            struct Period{
                int x,y;
            };
            smartstore<Period> periods;
            int waitIntervalOpening(int x){
                int opening=x;
                for(int i=0;i<periods.size();i++){
                    if(periods[i].x<=opening || periods[i].y>=opening){
                        opening=periods[i].y+1;
                    }
                }
                return opening;
            }
            void addInterval(int x,int y){
                Period p;
                p.x=x;
                p.y=y;

                periods.pushBack(p);
            }
        };

        int t=0;
        smartstore<Interval> rows(nrRows);
        for(int gi=0;gi<groups.size();gi++){
            for(int pi=0;pi<groups[gi].passangers.size();pi++){
                int timeP=ROW_TIME;
                for(int i=1;i<groups[gi].passangers[pi].row;i++){
                    int arrival=timeP;
                    timeP=rows[i].waitIntervalOpening(timeP)+ROW_TIME;
                    rows[i].addInterval(arrival,timeP);
                }
                t=max(t,timeP+groups[gi].passangers[pi].waitTime);
                rows[groups[gi].passangers[pi].row].addInterval(timeP,timeP+groups[gi].passangers[pi].waitTime);
            }
        }
        return t;
    }
    void Print(){
        smartstore<Passanger> pass;
        for(int g=0;g<groups.size();g++){
            for(int p=0;p<groups[g].passangers.size();p++){
                pass.pushBack(groups[g].passangers[p]);
            }
        } 
        pass.sort(sortByRowSeat);
        for(int k=0;k<nrRows;k++){
            for(int i=0;i<nrSeats;i++){
                cout<<pass[i].groupID<<" ";
                if(i==nrSeats/2-1){
                    cout<<"     ";
                }
            }
            cout<<endl;
        }
    }
};

Airplane& GenerateRandomAirplane(int nrRows,int nrSeats){
    Airplane a;
    srand(NULL);
    int nrGroups=rand()%(nrRows*nrSeats/MAX_GROUP_TO_SEATS_RATIO)+1;

    for(int i=0;i<nrRows;i++){
        for(int j=0;j<nrSeats;j++){
            //find random group
            int randomGroup=rand()%nrGroups+1;

            //give it the details
            Passanger current;
            current.row=i+1;
            current.seat=char('A'+j);
            current.groupID=randomGroup;
            current.waitTime=rand()%(MAX_WAIT_TIME-MIN_WAIT_TIME+1)+MIN_WAIT_TIME;
            
            //give it away
            a.groups[randomGroup].passangers.pushBack(current);
        }
    }
    return a;
}
Airplane& changePassangerInfo(Airplane& a){
    Airplane newA;
    newA.Copy(a);
    for(int gi=0;gi<newA.groups.size();gi++){
        for(int pi=0;pi<newA.groups[gi].passangers.size();pi++){
            newA.groups[gi].passangers[pi].waitTime=rand()%(MAX_WAIT_TIME-MIN_WAIT_TIME+1)+MIN_WAIT_TIME;
        }
    }
    return newA;
}
