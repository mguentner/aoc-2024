#!/usr/bin/env python3
import copy

example = """
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"""

def input_to_matrix(input):
    return [ list(x) for x in input.splitlines() if len(x) > 0 ]

def find_cursor_in_matrix(matrix):
    for y, row in enumerate(matrix):
        for x, cell in enumerate(row):
            if cell == '^' or cell == '>' or cell == '<' or cell == 'v':
                return (cell, y, x)
    return None

def look_around(matrix, cursor):
    y = cursor[1]
    x = cursor[2]
    if cursor[0] == '^':
        if cursor[1] > 0:
            return (matrix[y-1][x], y-1, x)
        else:
            return (None, y-1, x)
    if cursor[0] == 'v':
        if cursor[1] < len(matrix)-1:
            return (matrix[y+1][x], y+1, x)
        else:
            return (None, y+1, x)
    if cursor[0] == '<':
        if cursor[2] > 0:
            return (matrix[y][x-1], y, x-1)
        else:
            return (None, y, x-1)
    if cursor[0] == '>':
        if cursor[2] < len(matrix)-1:
            return (matrix[y][x+1], y, x+1)
        else:
            return (None, y, x+1)
        
def turn(cursor):
    if cursor[0] == "^":
        return ">"
    if cursor[0] == ">":
        return "v"
    if cursor[0] == "v":
        return "<"
    if cursor[0] == "<":
        return "^"

def move(matrix, memory):
    cursor = find_cursor_in_matrix(matrix)
    (cell_in_direction, cell_in_direction_y, cell_in_direction_x) = look_around(matrix, cursor)
    if cell_in_direction == '#':
        if cursor[0] == '^':
            matrix[cursor[1]][cursor[2]] = '>'
        if cursor[0] == '>':
            matrix[cursor[1]][cursor[2]] = 'v'
        if cursor[0] == 'v':
            matrix[cursor[1]][cursor[2]] = '<'
        if cursor[0] == '<':
            matrix[cursor[1]][cursor[2]] = '^'
        return matrix, False, False
    elif memory[cursor[1]][cursor[2]] == cursor:
        return matrix, False, True
    elif cell_in_direction is not None:
        if cursor[0] == '^':
            matrix[cursor[1]-1][cursor[2]] = '^'
        if cursor[0] == 'v':
            matrix[cursor[1]+1][cursor[2]] = 'v'
        if cursor[0] == '<':
            matrix[cursor[1]][cursor[2]-1] = '<'
        if cursor[0] == '>':
            matrix[cursor[1]][cursor[2]+1] = '>'
        matrix[cursor[1]][cursor[2]] = 'X'
        memory[cursor[1]][cursor[2]] = cursor
        return matrix, False, False
    else:
        matrix[cursor[1]][cursor[2]] = 'X'
        return matrix, True, False
    
def print_matrix(matrix):
    for row in matrix:
        print(''.join(row))
    
def walk(orig_matrix):
    finished = False
    loop = False
    matrix = [ copy.deepcopy(x) for x in orig_matrix]
    memory = [ ['.' for y in range(len(matrix[0])) ] for x in range(len(matrix)) ]
    while not finished and not loop:
        matrix, finished, loop = move(matrix, memory)
    return matrix, loop

def get_positions(matrix):
    positions = []
    for y, row in enumerate(matrix):
        for x, cell in enumerate(row):
            if cell in 'X':
                positions.append((y,x))
    return positions

def find_blockers_in_matrix(orig_matrix, positions):
    possible_blockers = 0
    cursor = find_cursor_in_matrix(orig_matrix)
    for i, pos in enumerate(positions):
        print("pos {}/{}".format(i, len(positions)))
        y = pos[0]
        x = pos[1]
        if y == cursor[1] and x == cursor[2]:
            continue
        if cursor[0] == "#":
            continue
        matrix = [ [c for c in row ] for row in orig_matrix]
        matrix[y][x] = '#'
        walked, loop = walk(matrix)
        if loop:
            print_matrix(matrix)
            possible_blockers += 1
    return possible_blockers
            
example_matrix = input_to_matrix(example)

with open("input.txt", "r") as f:
    input_matrix = input_to_matrix(f.read())

example_walked, loop = walk(example_matrix)
example_positions = get_positions(example_walked)
print("positions length:", len(example_positions))

print(find_blockers_in_matrix(example_matrix, example_positions))

input_walked, loop = walk(input_matrix)
input_positions = get_positions(input_walked)
print(len(input_positions))
print(find_blockers_in_matrix(input_matrix, input_positions))