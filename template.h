#include <stdexcept>
#include<algorithm>
using namespace std;
template<typename T> class smartstore{
    private:
        T* data;
        int length,capacity;
    public:
        smartstore<T>(){
            data=new T;
            length=0;
            capacity=1;
        }
        smartstore<T>(int l){
            data=new T[l];
            length=l;
            capacity=l;
        }
        T& operator[](int i){
           if(i<length){
            return data[i];
           }else{
            throw runtime_error("Index out of range");
           }
        }
        void operator=(smartstore<T>& other){
            data=other.data;
            length=other.length;
            capacity=other.capacity;
        }
        void pushBack(T &t){
            if(length==capacity){
                T* newData=new T[capacity*2];
                capacity*=2;
                for(int i=0;i<length;i++){
                    newData[i]=data[i];
                }
            }
            data[length]=*t;
            length++;
        }
        bool empty(){
            return length==0;
        }
        T popBack(){
            if(length>0){
                length--;
            }
            return data[length];
        }
        int size(){
            return length;
        }
        T cutout(int index){
            if(index<length){
                T keep=data[index];
                for(int i=index;i<length-1;i++){
                    data[i]=data[i+1];
                }
                length--;
                return keep;
            }else{
                throw runtime_error("Index out of range");                
            }
        }
        void swap(int i,int j){
            T rem=data[i];
            data[i]=data[j];
            data[j]=rem;
        }
        int findSmallest(){
            int indx=0;
            for(int i=0;i<length;i++){
                if(data[indx]>data[i]){
                    indx=i;
                }
            }
            return indx;
        }
        void sort(bool(*sorting)(T&,T&)){
            sort(data,data+length-1,sorting);
        }
        void clear(){
            lenght=0;
        }
        smartstore<T>& copy(){
            smartstore<T> c;
            for(int i=0;i<length;i++){
                c.pushBack(data[i]);
            }            
            return c;
        }
};