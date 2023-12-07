import re

def get_numbers(lines):
    numbers = []
    for line in lines:
        numbers.append("".join(re.findall(r'\d+', line)))
    for i, number in enumerate(numbers):
        numbers[i] = int(number[0] + number[-1])
    return numbers


if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    numbers = get_numbers(lines)
    print(sum(numbers))
    lines = ";".join(lines)
    lines = lines.replace("one", "one1one") \
        .replace("two", "two2two") \
        .replace("three", "three3three") \
        .replace("four", "four4four") \
        .replace("five", "five5five") \
        .replace("six", "six6six") \
        .replace("seven", "seven7seven") \
        .replace("eight", "eight8eight") \
        .replace("nine", "nine9nine")
    numbers = get_numbers(lines.split(";"))
    print(sum(numbers))