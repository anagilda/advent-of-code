import unittest
from solution import (
    complete_gravity_assist,
    intcode_computer,
)


class Test1202ProgramAlarm(unittest.TestCase):

    def test_restore_gravity_assist_program(self):
        self.assertEqual([2, 0, 0, 0, 99], intcode_computer([1, 0, 0, 0, 99]))
        self.assertEqual([2, 3, 0, 6, 99], intcode_computer([2, 3, 0, 3, 99]))
        self.assertEqual([2, 4, 4, 5, 99, 9801], intcode_computer([2, 4, 4, 5, 99, 0]))
        self.assertEqual([30, 1, 1, 4, 2, 5, 6, 0, 99], intcode_computer([1, 1, 1, 4, 99, 5, 6, 0, 99]))
        self.assertEqual([3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
                         intcode_computer([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]))

    def test_complete_gravity_assist(self):
        with open('input.txt') as input_file:
            program = [int(intcode) for intcode in input_file.read().split(',')]

        self.assertEqual(1202, complete_gravity_assist(program=program, output=4330636))
