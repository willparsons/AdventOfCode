def main():
    horizontal, depth = 0, 0

    with open("input.txt", "r") as file:
        for line in file:
            command, value = line.rstrip().split(" ")
            value = int(value)

            if command == "forward":
                horizontal += value
            elif command == "up":
                depth -= value
            elif command == "down":
                depth += value

    print(horizontal * depth)


if __name__ == "__main__":
    main()
