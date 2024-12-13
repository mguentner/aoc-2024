#!/usr/bin/env python3
example = """
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"""

def input_to_matrix(input):
    return [ list(x) for x in input.splitlines() if len(x) > 0 ]

def neighbors(y, x, max_y, max_x):
    res = []
    for [dy, dx] in [ (1,0), (-1,0), (0,-1), (0,1) ]:
        if y + dy > max_y or x + dx > max_x or y + dy < 0 or x + dx < 0:
            continue
        res.append(( y + dy, x + dx ))
    return res

def get_group(y, x, max_y, max_x, coords):
    n = neighbors(y, x, max_y, max_x)
    res = set()
    for neighbor in n:
        if neighbor in coords:
            res.add(neighbor)
            for g in get_group(neighbor[0], neighbor[1], max_y, max_x, [ x for x in coords if x not in res ]):
                res.add(g)
    return res

def find_groups(matrix, letter_coords):
    groups = []
    while True:
        if letter_coords == []:
            break
        coord = letter_coords.pop()
        group = get_group(coord[0], coord[1], len(matrix), len(matrix[0]), letter_coords)
        group.add(coord)
        groups.append(group)
        for g in group:
            if g in letter_coords:
               letter_coords.remove(g)
    return groups

def calculate_perimeter_part1(group, max_y, max_x):
    perimeter = 0
    for [y, x] in group:
        for [dy, dx] in [ (1,0), (-1,0), (0,-1), (0,1) ]:
            if (y+dy,x+dx) not in group:
                perimeter += 1
    return perimeter

def calculate_perimeter_part2(group, max_y, max_x):
    return 0

def walk(matrix):
    letters = set([ item for row in matrix for item in row ])
    cost1 = 0
    cost2 = 0
    for letter in letters:
        letter_coords = [ (y,x) for y in range(len(matrix)) for x in range(len(matrix[0])) if matrix[y][x] == letter ]
        letter_groups = find_groups(matrix, letter_coords)
        letter_cost1 = 0
        letter_cost2 = 0
        for group in letter_groups:
            perimeter_cost1 = calculate_perimeter_part1(group, len(matrix), len(matrix[0]))
            perimeter_cost2 = calculate_perimeter_part2(group, len(matrix), len(matrix[0]))
            group_cost1 = len(group) * perimeter_cost1
            group_cost2 = len(group) * perimeter_cost2
            #print("letter: {}, cost: {},  len {} perimeter1 {} perimeter 2 {}".format(letter, group_cost1, len(group), perimeter_cost1, perimeter_cost2))
            letter_cost1 += group_cost1
            letter_cost2 += group_cost2
        cost1 += letter_cost1
        cost2 += letter_cost2
    return [cost1, cost2]

example_matrix = input_to_matrix(example)
print("example:", walk(example_matrix))

with open("input.txt", "r") as f:
    input_matrix = input_to_matrix(f.read())
print("input: ", walk(input_matrix))


