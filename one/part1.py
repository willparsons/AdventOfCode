def main():
    prev = -1
    count = 0

    with open("input.txt", "r") as file:
        for line in file:

            num = int(line)

            if prev == -1:
                prev = num

            if prev < num:
                count += 1

            prev = num

    print(count)


if __name__ == "__main__":
    main()
