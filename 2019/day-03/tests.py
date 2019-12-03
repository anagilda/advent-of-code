import unittest

from solution import (
    manhattan_distance_to_closest_intersection,
    steps_to_closest_intersection)


class TestCrossedWires(unittest.TestCase):

    def test_install_fuel_management_system_with_manhattan_distance(self):
        self.assertEqual(6, manhattan_distance_to_closest_intersection(
            green_wire_path=['R8', 'U5', 'L5', 'D3'],
            blue_wire_path=['U7', 'R6', 'D4', 'L4'],
        ))
        self.assertEqual(159, manhattan_distance_to_closest_intersection(
            green_wire_path=['R75', 'D30', 'R83', 'U83', 'L12', 'D49', 'R71', 'U7', 'L72'],
            blue_wire_path=['U62', 'R66', 'U55', 'R34', 'D71', 'R55', 'D58', 'R83'],
        ))
        self.assertEqual(135, manhattan_distance_to_closest_intersection(
            green_wire_path=['R98', 'U47', 'R26', 'D63', 'R33', 'U87', 'L62', 'D20', 'R33', 'U53', 'R51'],
            blue_wire_path=['U98', 'R91', 'D20', 'R16', 'D67', 'R40', 'U7', 'R15', 'U6', 'R7'],
        ))

    def test_install_fuel_management_system_with_steps(self):
        self.assertEqual(30, steps_to_closest_intersection(
            green_wire_path=['R8', 'U5', 'L5', 'D3'],
            blue_wire_path=['U7', 'R6', 'D4', 'L4'],
        ))
        self.assertEqual(610, steps_to_closest_intersection(
            green_wire_path=['R75', 'D30', 'R83', 'U83', 'L12', 'D49', 'R71', 'U7', 'L72'],
            blue_wire_path=['U62', 'R66', 'U55', 'R34', 'D71', 'R55', 'D58', 'R83'],
        ))
        self.assertEqual(410, steps_to_closest_intersection(
            green_wire_path=['R98', 'U47', 'R26', 'D63', 'R33', 'U87', 'L62', 'D20', 'R33', 'U53', 'R51'],
            blue_wire_path=['U98', 'R91', 'D20', 'R16', 'D67', 'R40', 'U7', 'R15', 'U6', 'R7'],
        ))
