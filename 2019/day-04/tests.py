import unittest

from solution import (
    is_a_possible_venus_fuel_depot_password,
    possible_venus_fuel_depot_passwords_in_list,
)


class TestSecureContainer(unittest.TestCase):

    def test_unlock_venus_fuel_depot_without_post_it_note(self):
        self.assertEqual(True, is_a_possible_venus_fuel_depot_password(password=122345))
        self.assertEqual(True, is_a_possible_venus_fuel_depot_password(password=111123))
        self.assertEqual(True, is_a_possible_venus_fuel_depot_password(password=111111))
        self.assertEqual(False, is_a_possible_venus_fuel_depot_password(password=135679))
        self.assertEqual(False, is_a_possible_venus_fuel_depot_password(password=223450))
        self.assertEqual(False, is_a_possible_venus_fuel_depot_password(password=123789))

    def test_unlock_venus_fuel_depot_new_elf_memory(self):
        self.assertEqual(True, is_a_possible_venus_fuel_depot_password(password=112233, strict_repeat=True))
        self.assertEqual(True, is_a_possible_venus_fuel_depot_password(password=111122, strict_repeat=True))
        self.assertEqual(False, is_a_possible_venus_fuel_depot_password(password=123444, strict_repeat=True))
