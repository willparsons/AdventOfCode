from typing import List


def get_rate(input: List[str], use_most_common: bool, index: int = 0):
    if (len(input) == 1):
        return int(input[0], 2)

    zeroes, ones = [], []

    for val in input:
        if int(val[index]) == 0:
            zeroes.append(val)
        else:
            ones.append(val)

    if use_most_common:
        most_common_list = ones if len(ones) >= len(zeroes) else zeroes
        return get_rate(most_common_list, use_most_common, index+1)
    else:
        least_common_list = zeroes if len(zeroes) <= len(ones) else ones
        return get_rate(least_common_list, use_most_common, index+1)


def main():
    with open("input.txt", "r") as file:
        input = [i.strip() for i in file.readlines()]

    oxygen_rate = get_rate(input, True)
    scrubber_rate = get_rate(input, False)

    print(oxygen_rate * scrubber_rate)


if __name__ == "__main__":
    main()
