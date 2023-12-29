from random import Random

seed: int = 5489
r = Random(x=seed)

with open("init_default_state.txt", "w", encoding="utf-8") as f:
    for num in r.getstate()[1]:
        f.write(f"{num}\n")