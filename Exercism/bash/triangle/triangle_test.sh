#!/usr/bin/env bash

# local version: 1.2.1.0

# Test returns true if the triangle is equilateral

@test "all sides are equal, equilateral" {
  run bash triangle.sh equilateral 2 2 2
  (( status == 0 ))
  [[ $output == "true" ]]
}

@test "any side is unequal" {
  run bash triangle.sh equilateral 2 3 2
  (( status == 0 ))
  [[ $output == "false" ]]
}

@test "no sides are equal, equilateral" {
  run bash triangle.sh equilateral 5 4 6
  (( status == 0 ))
  [[ $output == "false" ]]
}

@test "all zero sides is not a triangle" {
  run bash triangle.sh equilateral 0 0 0
  (( status == 0 ))
  [[ $output == "false" ]]
}

# Bonus: deal with floats

@test "sides may be floats, equilateral" {
  run bash triangle.sh equilateral 0.5 0.5 0.5
  (( status == 0 ))
  [[ $output == "true" ]]
}

# Test returns true if the triangle is isosceles

@test "last two sides are equal" {
  run bash triangle.sh isosceles 3 4 4
  (( status == 0 ))
  [[ $output == "true" ]]
}

@test "first two sides are equal" {
  run bash triangle.sh isosceles 4 4 3
  (( status == 0 ))
  [[ $output == "true" ]]
}

@test "first and last sides are equal" {
  run bash triangle.sh isosceles 4 3 4
  (( status == 0 ))
  [[ $output == "true" ]]
}

@test "equilateral triangles are also isosceles" {
  run bash triangle.sh isosceles 4 4 4
  (( status == 0 ))
  [[ $output == "true" ]]
}

@test "no sides are equal, isosceles" {
  run bash triangle.sh isosceles 2 3 4
  (( status == 0 ))
  [[ $output == "false" ]]
}

@test "first triangle inequality violation" {
  run bash triangle.sh isosceles 1 1 3
  (( status == 0 ))
  [[ $output == "false" ]]
}

@test "second triangle inequality violation" {
  run bash triangle.sh isosceles 1 3 1
  (( status == 0 ))
  [[ $output == "false" ]]
}

@test "third triangle inequality violation" {
  run bash triangle.sh isosceles 3 1 1
  (( status == 0 ))
  [[ $output == "false" ]]
}

# Bonus: deal with floats

@test "sides may be floats, isosceles" {
  run bash triangle.sh isosceles 0.5 0.4 0.5
  (( status == 0 ))
  [[ $output == "true" ]]
}

# Test returns true if the triangle is scalene

@test "no sides are equal, scalene" {
  run bash triangle.sh scalene 5 4 6
  (( status == 0 ))
  [[ $output == "true" ]]
}

@test "all sides are equal, scalene" {
  run bash triangle.sh scalene 4 4 4
  (( status == 0 ))
  [[ $output == "false" ]]
}

@test "two sides are equal" {
  run bash triangle.sh scalene 4 4 3
  (( status == 0 ))
  [[ $output == "false" ]]
}

@test "may not violate triangle inequality" {
  run bash triangle.sh scalene 7 3 2
  (( status == 0 ))
  [[ $output == "false" ]]
}

# Bonus: deal with floats

@test "sides may be floats, scalene" {
  run bash triangle.sh scalene 0.5 0.4 0.6
  (( status == 0 ))
  [[ $output == "true" ]]
}
