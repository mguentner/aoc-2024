#!/usr/bin/env python3
import re
import math

example = """
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"""

def parse_input(value):
    res = []
    a = [0,0]
    b = [0,0]
    b_regex = r"Button (A|B): X\+(\d+), Y\+(\d+)"
    p_regex = r"Prize: X=(\d+), Y=(\d+)"
    for line in value.splitlines():
        a_m = re.search(b_regex, line)
        if a_m and a_m.group(1) == "A":
            a = [int(a_m.group(2)), int(a_m.group(3))]
        if a_m and a_m.group(1) == "B":
            b = [int(a_m.group(2)), int(a_m.group(3))]
        p_m = re.search(p_regex, line)
        if p_m:
            res.append([a,b,[int(p_m.group(1)), int(p_m.group(2))]])
            a = [0,0]
            b = [0,0]
    return res

def gauss(equation):
    matrix = [
        [equation[0][0], equation[1][0], equation[2][0]],
        [equation[0][1], equation[1][1], equation[2][1]]
    ]
    if matrix[0][0] == 0:
        matrix = [matrix[1][matrix[0]]]
        if matrix[0][0] == 0:
            return None
    div = matrix[0][0]
    matrix[0] = [ x/div for x in matrix[0] ]
    
    mul = matrix[1][0]
    add = [ x*mul for x in matrix[0] ]
    matrix[1] = [ x-add[i] for (i, x) in enumerate(matrix[1])]
    div = matrix[1][1]
    matrix[1] = [ x/div for x in matrix[1]]
    mul = matrix[0][1]
    add = [ x*mul for x in matrix[1] ]
    matrix[0] = [ x-add[i] for (i, x) in enumerate(matrix[0])]
    
    a = round(matrix[0][2])
    b = round(matrix[1][2])
    if abs(a-matrix[0][2]) < 0.001 and abs(b-matrix[1][2]) < 0.001:
        return [a,b]
    return None

def calculate_cost(equations):
    cost = 0
    for eq in equations:
        solution = gauss(eq)
        if solution:
            cost += solution[0] * 3 + solution[1]
    return cost
    
example_equations = parse_input(example)

with open("input.txt", "r") as f:
    input_equations = parse_input(f.read())

print("cost example part1:", calculate_cost(example_equations))
print("cost input part1:", calculate_cost(input_equations))

print("cost input part2:", calculate_cost([ [eq[0],eq[1], [eq[2][0]+10000000000000, eq[2][1]+10000000000000]] for eq in input_equations]))

               