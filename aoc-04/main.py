#!/usr/bin/env python3

example = """
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"""

def input_to_matrix(input):
    return [ x for x in input.splitlines() if len(x) > 0 ]

def check_horizontal(matrix, row, col):
    tmp = ""
    for c in range(4):
        if col + c < len(matrix[0]):
            tmp += matrix[row][col + c]
    if tmp == "XMAS":
        return True
    if tmp == "SAMX":
        return True

    return False

def check_vertical(matrix, row, col):
    tmp = ""
    for r in range(4):
        if row + r < len(matrix):
            tmp += matrix[row + r][col]
    if tmp == "XMAS":
        return True
    if tmp == "SAMX":
        return True
    return False

def check_diagonal(matrix, row, col):
    tmp = ""
    for x in range(4):
        if row + x < len(matrix) and col + x < len(matrix[0]):
            tmp += matrix[row + x][col + x]
    if tmp == "XMAS":
        return True
    if tmp == "SAMX":
        return True
    return False

def check_diagonal2(matrix, row, col):
    tmp = ""
    for x in range(4):
        if row + x < len(matrix) and (col - x) >= 0:
            tmp += matrix[row + x][col - x]
    if tmp == "XMAS":
        return True
    if tmp == "SAMX":
        return True
    return False

def check_all(matrix, row, col):
    sum = 0
    if check_horizontal(matrix, row, col):
        sum += 1
    if check_vertical(matrix, row, col):
        sum += 1
    if check_diagonal(matrix, row, col):
        sum += 1
    if check_diagonal2(matrix, row, col):
        sum += 1

    return sum

def find_xmas(matrix):
    total_sum = 0
    for r in range(len(matrix)):
        for c in range(len(matrix[0])):
            total_sum += check_all(matrix, r, c)
    return total_sum


def find_xmasx(matrix):
    sum = 0
    for r in range(len(matrix)):
        for c in range(len(matrix[0])):
            if r+2 < len(matrix) and c+2 < len(matrix[0]):
                if matrix[r+1][c+1] != "A":
                    continue
                if not ((matrix[r][c] == "M" and matrix[r+2][c+2] == "S") or (matrix[r][c] == "S" and matrix[r+2][c+2] == "M")):
                    continue
                if not ((matrix[r][c+2] == "M" and matrix[r+2][c] == "S") or (matrix[r][c+2] == "S" and matrix[r+2][c] == "M")):
                    continue
                sum += 1
    return sum


def main():
    example_matrix = input_to_matrix(example)

    with open("input.txt", "r") as f:
        input_matrix = input_to_matrix(f.read())

    print("xmas example", find_xmas(example_matrix))
    print("xmas input", find_xmas(input_matrix))

    print("x-mas example", find_xmasx(example_matrix))
    print("x-mas input", find_xmasx(input_matrix))

if __name__ == "__main__":
    main()