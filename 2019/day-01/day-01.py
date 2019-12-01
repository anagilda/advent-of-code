from typing import List
from math import floor


def read_components_mass(path: str):
    with open(path) as input_file:
        return [int(line.rstrip('\n')) for line in input_file]

def compute_fuel_by_mass(mass: int):
    fuel_required = floor(mass / 3) - 2
    return max(fuel_required, 0)

def components_fuel_requirements(components_mass: List[int], compute_extra_fuel: bool = False):
    components_fuel = [compute_fuel_by_mass(mass) for mass in components_mass]
    if compute_extra_fuel:
        components_fuel = [fuel + extra_fuel_requirements(fuel) for fuel in components_fuel]
    return sum(components_fuel)

def extra_fuel_requirements(component_fuel: int):
    total = 0
    fuel_for_fuel_mass = compute_fuel_by_mass(component_fuel)

    while fuel_for_fuel_mass > 0:
        total += fuel_for_fuel_mass

        fuel_for_fuel_mass = compute_fuel_by_mass(fuel_for_fuel_mass)

    return total

def total_fuel_requirements(components_mass: List[int]):
    return components_fuel_requirements(components_mass=components_mass, compute_extra_fuel=True)


if __name__ == '__main__':
    components_mass = read_components_mass('input.txt')

    """
    PART 1
    """
    fuel_requirements_task_1 = components_fuel_requirements(components_mass)

    """
    PART 2
    """
    fuel_requirements_task_2 = total_fuel_requirements(components_mass)

    print(fuel_requirements_task_1)
    print(fuel_requirements_task_2)
