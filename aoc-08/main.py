#!/usr/bin/env python3
import itertools
import math
import copy

example = """
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"""


def input_to_matrix(input):
    return [ list(x) for x in input.splitlines() if len(x) > 0 ]

input_to_matrix(example)

def find_coordinates(matrix):
    res = {}
    for y, row in enumerate(matrix):
        for x, cell in enumerate(row):
            if cell != '.':
                if cell in res:
                    res[cell].append((y,x))
                else:
                    res[cell] = [(y,x)]
    return res

def vector_between(a, b):
    return (b[0] - a[0], b[1] - a[1])

def add_vector(a, b):
    return (a[0] + b[0], a[1] + b[1])

def subtract_vector(a, b):
    return (a[0] - b[0], a[1] - b[1])

def multiply_vector(v, n):
    return (n * v[0], n * v[1])

def is_with_in_matrix(pos, matrix):
    return pos[0] >= 0 and pos[0] < len(matrix) and pos[1] >= 0 and pos[1] < len(matrix[pos[0]])

def find_anti_coords_part1(matrix):
    coords = find_coordinates(matrix)
    for element, coords in coords.items():
        for combo in itertools.combinations(coords, r=2):
            delta = vector_between(combo[0], combo[1])
            anti_a = subtract_vector(combo[0], delta)
            anti_b = add_vector(combo[1], delta)
            if is_with_in_matrix(anti_a, matrix):
                yield anti_a
            if is_with_in_matrix(anti_b, matrix):
                yield anti_b
                
def find_anti_coords_part2(matrix):
    coords = find_coordinates(matrix)
    for element, coords in coords.items():
        for combo in itertools.combinations(coords, r=2):
            delta = vector_between(combo[0], combo[1])
            count = 1
            yield combo[0]
            yield combo[1]
            while True:
                anti_a = subtract_vector(combo[0], multiply_vector(delta, count))
                anti_b = add_vector(combo[1], multiply_vector(delta, count))
                count += 1
                if is_with_in_matrix(anti_a, matrix):
                    yield anti_a
                if is_with_in_matrix(anti_b, matrix):
                    yield anti_b
                if not is_with_in_matrix(anti_a, matrix) and not is_with_in_matrix(anti_b, matrix):
                    break

def print_matrix(matrix):
    for row in matrix:
        print(''.join(row))
        

with open("input.txt", "r") as f:
    input_matrix = input_to_matrix(f.read())
example_matrix = input_to_matrix(example)

print("part1 example", len(set(find_anti_coords_part1(example_matrix))))
print("part1 input", len(set(find_anti_coords_part1(input_matrix))))

output_matrix = [ copy.deepcopy(x) for x in example_matrix ]
for a in set(find_anti_coords_part2(example_matrix)):
    if example_matrix[a[0]][a[1]] == '.':
        output_matrix[a[0]][a[1]] = '#'

print_matrix(output_matrix)

print("part2 example", len(set(find_anti_coords_part2(example_matrix))))
print("part2 input", len(set(find_anti_coords_part2(input_matrix))))