import time
import os

start_time = time.time()

script_dir = os.path.dirname(os.path.abspath(__file__))
file = os.path.join(script_dir, "input_d5_25.txt")
#file = os.path.join(script_dir, "tmp.txt")

fresh_ranges = []
ingredients = []
with open(file, "r", encoding="utf8") as f:
    lines = f.readlines()
    
for line in lines:
    line = line.strip()  
    if "-" in line:
        start, end = line.split("-")
        fresh_ranges.append((int(start), int(end)))
    elif line:
        ingredients.append(int(line))


def is_fresh(ingredient: int) -> bool:
    for start, end in fresh_ranges:
        if start <= ingredient <= end:
            return True
    return False


def count_valid_ids(ranges):
    merged = []
    for s, e in ranges:
        if not merged or s > merged[-1][1] + 1:
            merged.append([s, e])
        else:
            merged[-1][1] = max(merged[-1][1], e)

    return sum(e - s + 1 for s,e  in merged)


fresh_count = 0
for ingredient in ingredients:
    if is_fresh(ingredient):
        fresh_count += 1
ranges= sorted((min(start,end), max(start,end)) for start, end in fresh_ranges)
total_fresh_ids = count_valid_ids(ranges)

end_time = time.time()
print(f"Number of fresh ingredients: {fresh_count}")
print(f"Total number of fresh ingredient IDs: {total_fresh_ids}")
print(f"Execution time: {end_time - start_time:.6f} seconds")