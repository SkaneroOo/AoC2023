chars = "1234567890.\n"

def find_symbols(lines):
    for row, line in enumerate(lines):
        for col, char in enumerate(line):
            if char not in chars:
                yield row, col

def get_number_spans(lines):
    col_start = -1
    col_end = -1
    for row, line in enumerate(lines):
        for col, char in enumerate(line):
            if char.isdigit():
                if col_start == -1:
                    col_start = col
                col_end = col
            else:
                if col_start != -1:
                    yield row, (col_start, col_end), int(line[col_start:col_end + 1])
                    col_start = -1
                    col_end = -1

def check_neighborhood(numbers, row, col):
    for n in numbers:
        if abs(row - n[0]) <= 1:
            if col + 1 >= n[1][0] and col - 1 <= n[1][1]:
                yield n

if __name__ == "__main__":
    with open("input.txt", "r") as f:
        lines = f.readlines()

        final = set()

        numbers = [n for n in get_number_spans(lines)]

        total = 0
        true_total = 0
        for row, col in find_symbols(lines):
            for n in check_neighborhood(numbers, row, col):
                final.add(n)

            if lines[row][col] == "*":
                n = [i for i in check_neighborhood(numbers, row, col)]
                if len(n) == 2:
                    true_total += n[0][2] * n[1][2]
        
        for n in final:
            total += n[2]
        print(total)
        print(true_total)