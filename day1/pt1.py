INPUT = "input.txt"

with open(INPUT, "r") as f:
    input_lines = [line.strip() for line in f.readlines() if line.strip()]


class Lock:
    def __init__(self, init_pos: int = 50):
        self.pos = init_pos
        self.zeroes = 0

    def R(self, n: int):
        # print(f"Turning RIGHT by {n}")
        self.pos += n
        while self.pos > 99:
            self.pos -= 100
        # print(f"  ending position {self.pos}")
        if self.pos == 0:
            # print("adding zero")
            self.zeroes += 1

    def L(self, n: int):
        # print(f"Turning LEFT by {n}")
        self.pos -= n
        while self.pos < 0:
            self.pos += 100
        # print(f"  ending position {self.pos}")
        if self.pos == 0:
            # print("adding zero")
            self.zeroes += 1

lock = Lock()
for ins in input_lines:
    n = int(ins[1:])
    if ins[0] == "L":
        lock.L(n)
    if ins[0] == "R":
        lock.R(n)

print(lock.zeroes)
