#include<data.h>

const int nrRows=50;
const int nrSeats=6;

const int maxMutations=10;

void GenteticAlgorithm(int nrGenerations,int nrTopPerformers,int generationSize){
    srand(NULL);

    smartstore<Airplane> generation,topPerformers;
    for(int i=0;i<generationSize;i++){
        generation.pushBack(GenerateRandomAirplane(nrRows,nrSeats));
    }

    smartstore<int>performance;
    for(int g=0;g<nrGenerations;g++){
        performance.clear();
        for(int i=0;i<generationSize;i++){
            int perf=generation[i].Performance();
            performance.pushBack(perf);
        }

        topPerformers.clear();
        for(int i=0;i<nrTopPerformers;i++){
            int indexBiggest=performance.findSmallest();
            int perf=performance.cutout(indexBiggest);

            topPerformers.pushBack(generation[indexBiggest]);

            if(g==nrGenerations-1){
                cout<<i-1<<": "<<perf<<"\n";
                generation[indexBiggest].Print();
            }
        }
        
        generation.clear();
        for(int i=0;i<nrTopPerformers;i++){
            int mutationDegree=rand()%100+1;
            smartstore<Airplane> mutations=topPerformers[i].Mutate(maxMutations,mutationDegree,generationSize/nrTopPerformers);
            for(int j=0;j<generationSize/nrTopPerformers;j++){
                generation.pushBack(changePassangerInfo(mutations[j]));
            }
        }
    }

    
}

int main(){
    GenteticAlgorithm(100,10,100);
}