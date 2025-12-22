import time
import os
import pandas as pd


start_time = time.time()

script_dir = os.path.dirname(os.path.abspath(__file__))
file = os.path.join(script_dir, "input_d6_25.txt")
#file = os.path.join(script_dir, "tmp.txt")

data = pd.read_csv(file, header=None, sep="\\s+")

data_t = data.transpose()


def process_column(action,col):
    if action == "+":
        return sum(int(value) for value in col)
    elif action == "*":
        result = 1
        for value in col:
            result *= int(value)
        return result
    else:
        return None
    
sum_results = 0

#extract the last column of data_t and remove it from data_t
all_actions= data_t.iloc[:,-1]
new_data = data_t.iloc[:,:-1]

#enumerate through each row of new_data
for index, row in new_data.iterrows():
    action =all_actions.iloc[index]
    col_result = process_column(action, row)
    if col_result is not None:
        sum_results += col_result

print(f"Sum of all processed column pt1: {sum_results}")

with open(file, "r", encoding="utf-8") as f:
    lines = [ln.rstrip("\n") for ln in f if ln.rstrip("\n") != ""]

width = max(len(ln) for ln in lines)
grid = [list(ln.ljust(width)) for ln in lines]  #ljust to pad with spaces to make complete grid
   
data = pd.DataFrame(grid)                              
data_t = data.transpose()                                

all_actions = data_t.iloc[:, -1]    
new_data = data_t.iloc[:, :-1]      

def weird_math(new_data, all_actions):
    total = 0
    current_nums = []
    # iterate columns in reverse
    for col_idx in range(len(new_data) - 1, -1, -1):
        col = new_data.iloc[col_idx]          
        action = all_actions.iloc[col_idx]   
        # skips go to the next problem
        if (col == " ").all() and action == " ":
            continue

        # build the number 
        digits = [ch for ch in col.tolist() if isinstance(ch, str) and ch.isdigit()]
        if digits:
            current_nums.append(int("".join(digits)))
        if action == " ":
            continue
        res = process_column(action, current_nums)
        if res:
            total += res
        current_nums = []  

    return total

sum_results = weird_math(new_data, all_actions)
end_time = time.time()
print(f"Weird math: {sum_results}")
print(f"Execution time: {end_time - start_time:.6f} seconds")

