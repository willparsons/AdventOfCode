def main():
    count = 0

    input = []

    with open("input.txt", "r") as file:
        input = [int(x) for x in file.readlines()]

    for i in range(3, len(input)):
        prev = sum(input[i-3:i])
        current = sum(input[i-2:i+1])

        if prev < current:
            count += 1

    print(count)


if __name__ == "__main__":
    main()
