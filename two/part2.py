def main():
    horizontal, depth, aim = 0, 0, 0

    with open("input.txt", "r") as file:
        for line in file:
            command, value = line.rstrip().split(" ")
            value = int(value)

            if command == "forward":
                horizontal += value
                depth += aim * value
            elif command == "up":
                aim -= value
            elif command == "down":
                aim += value

    print(horizontal * depth)


if __name__ == "__main__":
    main()
