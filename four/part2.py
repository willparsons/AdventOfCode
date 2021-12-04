from typing import List


class Board:

    def __init__(self) -> None:
        self.data: List[List[dict[str, int]]] = []

    def add(self, row: List[str]):
        self.data.append([{i: 0} for i in row])

    def find_index_of(self, input: int) -> "tuple[int, int]":
        for i, row in enumerate(self.data):
            for j, column in enumerate(row):
                if column == input:
                    return i, j

    def mark(self, input: int):
        for i, row in enumerate(self.data):
            for j, column in enumerate(row):
                if input in column:
                    self.data[i][j][input] = 1

    def has_won(self) -> bool:
        for _, row in enumerate(self.data):
            if self.all_marked(row):
                return True

        for _, row in enumerate(zip(*self.data)):
            if self.all_marked(row):
                return True

        return False

    def all_marked(self, input) -> bool:
        count = 0
        for d in input:
            if 1 in d.values():
                count += 1
        return count == len(input)

    def sum_unmarked(self) -> int:
        count = 0
        for i, row in enumerate(self.data):
            for j, column in enumerate(row):
                if 0 in column.values():
                    for key in column.keys():
                        count += int(key)
        return count


with open("input.txt", "r") as file:
    order = file.readline().strip().split(",")

    boards: List[Board] = []
    board = Board()

    for line in file:
        if line.strip() == "":
            if (len(board.data)) != 0:
                boards.append(board)
                board = Board()
            continue

        l = line.strip().split(" ")
        while "" in l:
            l.remove("")

        board.add(l)

    boards.append(board)

winning_boards: List[Board] = []
for num in order:

    if num == "16":
        print()

    winner = False

    for b in boards:
        if b in winning_boards:
            continue

        b.mark(num)

        if b.has_won():
            winning_boards.append(b)

            if len(winning_boards) == len(boards):
                winner = True
                print(b.sum_unmarked()*int(num))

    if winner:
        break
