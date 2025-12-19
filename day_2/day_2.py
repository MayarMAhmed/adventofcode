
import pandas as pd
import time

start_time: float=time.time()
file="input_d2_25.txt"
#file="tmp"
data=pd.read_csv(file,header=None,sep=",").transpose()
data[['min','max']]=data[0].str.split("-",expand=True)

#need to find the 
def find_synynoms(min:int,max:int)->int:
    values: Any=0
    for i in range(min,max+1):
        str_i=str(i)
        if len(str_i)%2!=0:
            continue
        half=len(str_i)//2
        if str_i[:half]*2==str_i:
            values+=i
    return values
 
 
def find_duplicates(min:int,max:int)->int:
    values=0
    for i in range(min,max+1):
        val_str=str(i)
        for block_size in range(1, len(val_str)//2 + 1):
            if len(val_str) % block_size != 0:
                continue
            block = val_str[:block_size]
            repetitions = len(val_str) // block_size
            if block * repetitions == val_str:
                values+=i
                break
    return values
  
synynoms=0
duplicates=0
for index,row in data.iterrows():
    min=int(row['min'])
    max=int(row['max'])
    val=find_synynoms(min,max)
    synynoms+=val
    val2=find_duplicates(min,max)
    duplicates+=val2
    
    
print("The sum of all synynoms is:",synynoms)
print("The sum of all duplicates is:",duplicates)
print("Time taken:",time.time()-start_time)