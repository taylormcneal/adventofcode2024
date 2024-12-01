from collections import Counter

def read_input_file(filename):
    """
    Reads a file containing pairs of integers and returns two sorted lists
    of integers (left and right).
    """

    left_list, right_list = [], []

    with open(filename, "r") as file:
        for line in file:
            left, right = list(map(int, line.split()))
            left_list.append(left)
            right_list.append(right)

    left_list.sort()
    right_list.sort()

    return left_list, right_list

def calculate_difference(left_list, right_list):
    """
    Calculate the sum of differences between corresponding elements of two
    sorted lists.
    """

    return sum(abs(left - right) for left, right in zip(left_list, right_list))

def calculate_similarity(left_list, right_list):
    """
    Calculates the similarity score by summing the product of elements in the
    left list with the number of its occurences in the right list.
    """

    right_counter = Counter(right_list)
    return sum(left * right_counter[left] for left in left_list)

def solve_part1(filename):
    """
    Solves Day 1, Part 1: calculates the total difference between the two
    sorted lists.
    """

    left_list, right_list = read_input_file(filename)
    return calculate_difference(left_list, right_list)

def solve_part2(filename):
    """
    Solves Day 1, Part2: calculates the total similarity score between two
    sorted lists.
    """

    left_list, right_list = read_input_file(filename)
    return calculate_similarity(left_list, right_list)

def main():
    """
    Main: reads input, solves the two parts, and prints the answers.
    """

    filename="../input"

    part1_answer = solve_part1(filename)
    print(f"Part 1: {part1_answer}")

    part2_answer = solve_part2(filename)
    print(f"Part 2: {part2_answer}")

if __name__ == "__main__":
    main()
