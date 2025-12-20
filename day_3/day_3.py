from collections import deque
import time
import os
start_time: float=time.time()
# make the path relative
script_dir = os.path.dirname(os.path.abspath(__file__))
file = os.path.join(script_dir, "input_d3_25.txt")
#file="tmp.txt"

with open(file,"r",encoding="utf8") as f:
    data=f.readlines()
    
def find_combinations(line:str)->int:
    combinations=set()
    length=len(line)
    for i in range(length):
        for j in range(i+1,length):
            value=str(line[i])+str(line[j])
            combinations.add(value)
            
    val_typ= [int(x) for x in combinations]
    return max(val_typ)
                
def find_max_subseq(line:str,k:int=12)->int:
    length=len(line)
    remove=length-k
    stack=deque()
    
    for ch in line:
        while remove>0 and len(stack)>0 and stack[-1]<ch:
            stack.pop()
            remove-=1
        stack.append(ch)
    #if after the loop we still have to remove some digits, remove them from the end
    if remove > 0:
        stack = list(stack)[:-remove]
    vale = "".join(list(stack)[:k])
    return int(vale)

sum=0   
second_sum=0     
for line in data:
    sum+=find_combinations(line.strip())
    #sum+=find_max_subseq(line.strip(),2)
    second_sum+=find_max_subseq(line.strip(),12)
    

end_time=time.time()
print("The sum of the highest two digits in each line is:",sum)
print("The sum of the highest 12 digit combinations in each line is:",second_sum)
print("Time taken:",end_time-start_time)
