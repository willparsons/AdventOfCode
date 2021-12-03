def main():
    with open("input.txt", "r") as file:
        input = [i.strip() for i in file.readlines()]

    most_common = ""
    least_common = ""

    for bit_line in zip(*input):
        most_common += max(set(bit_line), key=bit_line.count)
        least_common += min(set(bit_line), key=bit_line.count)

    print(int(most_common, 2) * int(least_common, 2))


if __name__ == "__main__":
    main()
