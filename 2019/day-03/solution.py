from typing import List, Set, Tuple
from copy import deepcopy

CENTRAL_PORT_COORDINATES = (0, 0)

DIRECTION = {'U': (1, 0), 'D': (-1, 0), 'L': (0, -1), 'R': (0, 1)}


def read_wire_path_instructions(path: str) -> Tuple[List[str], ...]:
    with open(path) as input_file:
        return tuple(line.split(',') for line in input_file)


def wire_coordinates(wire_path: List[str],
                     initial_point: Tuple[int, int] = CENTRAL_PORT_COORDINATES) -> List[Tuple[int, int]]:
    current_coordinates = deepcopy(initial_point)
    coordinates = [current_coordinates]

    for path in wire_path:
        direction = path[0]
        steps = int(path[1:])

        for _ in range(steps):
            current_coordinates = add_xy_coordinates(point=current_coordinates, vector=DIRECTION[direction])
            coordinates.append(current_coordinates)

    return coordinates


def add_xy_coordinates(point: Tuple[int, int], vector: Tuple[int, int]) -> Tuple[int, ...]:
    return tuple(sum(coordinate) for coordinate in zip(point, vector))


def wire_intersection_points(green_wire_coordinates: List[Tuple[int, int]],
                             blue_wire_coordinates: List[Tuple[int, int]]) -> Set[Tuple[int, int]]:
    return set(green_wire_coordinates).intersection(blue_wire_coordinates)


def compute_manhattan_distance_for_coordinates(distant_coordinates: Tuple[int, int],
                                               central_port_coordinates: Tuple[int, int] = CENTRAL_PORT_COORDINATES,
                                               ) -> int:
    distance_vector = (abs(a - b) for a, b in zip(central_port_coordinates, distant_coordinates))
    return sum(distance_vector)


def minimum_distance_to_intersection(intersection_points: Set[Tuple[int, int]]):
    distances = [compute_manhattan_distance_for_coordinates(distant_coordinates=point)
                 for point in intersection_points
                 if point != CENTRAL_PORT_COORDINATES]

    return min(distances)


def manhattan_distance_to_closest_intersection(green_wire_path: List[str], blue_wire_path: List[str]) -> int:
    green_wire_coordinates = wire_coordinates(green_wire_path)
    blue_wire_coordinates = wire_coordinates(blue_wire_path)

    intersection_points = wire_intersection_points(
        green_wire_coordinates=green_wire_coordinates,
        blue_wire_coordinates=blue_wire_coordinates,
    )

    minimum_distance = minimum_distance_to_intersection(intersection_points=intersection_points)

    return minimum_distance


def compute_steps_for_coordinates(wire_coordinates_steps: List[Tuple[int, int]],
                                  distant_coordinates: Tuple[int, int],
                                  central_port_coordinates: Tuple[int, int] = CENTRAL_PORT_COORDINATES,
                                  ) -> int:
    wire_start_point_in_path_step = wire_coordinates_steps.index(central_port_coordinates) + 1
    wire_intersection_in_path_step = wire_coordinates_steps.index(distant_coordinates) + 1
    return wire_intersection_in_path_step - wire_start_point_in_path_step


def minimum_steps_to_intersection(green_wire_coordinates: List[Tuple[int, int]],
                                  blue_wire_coordinates: List[Tuple[int, int]],
                                  intersection_points: Set[Tuple[int, int]]) -> int:
    steps = [compute_steps_for_coordinates(wire_coordinates_steps=green_wire_coordinates, distant_coordinates=point)
             +
             compute_steps_for_coordinates(wire_coordinates_steps=blue_wire_coordinates, distant_coordinates=point)
             for point in intersection_points
             if point != CENTRAL_PORT_COORDINATES]

    return min(steps)


def steps_to_closest_intersection(green_wire_path: List[str], blue_wire_path: List[str]) -> int:
    green_wire_coordinates = wire_coordinates(green_wire_path)
    blue_wire_coordinates = wire_coordinates(blue_wire_path)

    intersection_points = wire_intersection_points(
        green_wire_coordinates=green_wire_coordinates,
        blue_wire_coordinates=blue_wire_coordinates,
    )

    minimum_steps = minimum_steps_to_intersection(
        green_wire_coordinates=green_wire_coordinates,
        blue_wire_coordinates=blue_wire_coordinates,
        intersection_points=intersection_points,
    )

    return minimum_steps


if __name__ == '__main__':
    green_wire, blue_wire = read_wire_path_instructions('input.txt')

    '''
    PART 1
    '''
    minimum_manhattan_distance = manhattan_distance_to_closest_intersection(green_wire_path=green_wire,
                                                                            blue_wire_path=blue_wire)

    print(minimum_manhattan_distance)

    '''
    PART 2
    '''
    minimum_steps_required = steps_to_closest_intersection(green_wire_path=green_wire, blue_wire_path=blue_wire)

    print(minimum_steps_required)
