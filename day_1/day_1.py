import time

start_time: float=time.time()
import os

script_dir = os.path.dirname(os.path.abspath(__file__))
file = os.path.join(script_dir, "input_d1_25.txt")
with open(file,encoding="utf-8") as file:
    data=file.readlines()

def move(direction:int,distance:int,intial_position:int) -> tuple[int,int]:
    r"""Move left or right from the initial position by the given distance.
    Args:
        direction (str): "L" for left, "R" for right
        distance (int): distance to move
        initial_position (int): starting position (0-99)
    Returns:
        tuple: (number of times crossed 0, new position)
    """
    counter=0
    if direction=="L":
        value=initial_position-distance
    elif direction=="R":
        value=initial_position+distance
    else:
        print("Invalid direction")
    #go in circle if the value is less than 0 or greater than 99
    while (value<0) or (value>99):
        counter+=1
        if value==100:
            value=0
        if value<0:
            value=100+(value)
        elif value>100:
            value=value-100
    return counter,value

initial_position=50
c=0

zeros=0
for line in data:
    line=line.strip()
    #split the line into characters
    chars=list(line)
    direction=chars[0]
    distance=int("".join(chars[1:]))
    tmp=move(direction,distance,initial_position)
    value=f"{direction}{distance}"
    initial_position=tmp[1]
    if initial_position==0:
        c+=1
    zeros+=tmp[0]
        
print(f"Final Position: {initial_position}")
print(f"Number of times landed on 0: {c}")
print(f"Total number of times crossed 0: {zeros}")
print("Time taken:",time.time()-start_time)
        
