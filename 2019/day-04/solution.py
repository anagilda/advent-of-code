from typing import List, Tuple


def read_password_range(path: str) -> Tuple[int, ...]:
    with open(path) as input_file:
        return tuple(int(limit) for limit in input_file.read().split('-'))


def window_of_adjacent_digits(iterable_number: str, window_size: int):
    return [iterable_number[index:index + window_size] for index in range(len(iterable_number) - (window_size - 1))]


def only_repeated_digits(iterable_number: str) -> bool:
    return all(digit == iterable_number[0] for digit in iterable_number)


def exists_window_of_repeated_digits(iterable_number: str, window_size: int, strict_repeat: bool = False) -> bool:
    possible_repeated_digits = window_of_adjacent_digits(iterable_number=iterable_number, window_size=window_size)

    if not strict_repeat:
        are_repeated_digits = (only_repeated_digits(iterable_number=number) for number in possible_repeated_digits)

        return any(are_repeated_digits)

    else:
        for index in range(len(possible_repeated_digits)):
            window = possible_repeated_digits[index]
            if only_repeated_digits(iterable_number=window):
                flag = True

                if index > 0:
                    if window[0] == possible_repeated_digits[index - 1][0]:
                        flag = False

                if index < len(possible_repeated_digits) - 1:
                    if window[0] == possible_repeated_digits[index + 1][-1]:
                        flag = False

                if flag:
                    return True

        return False


def are_numbers_not_decreasing(iterable_number: str) -> bool:
    is_number_not_decreasing_by_window = [window[0] <= window[1]
                                          for window in
                                          window_of_adjacent_digits(iterable_number=iterable_number, window_size=2)]
    return all(is_number_not_decreasing_by_window)


def is_a_possible_venus_fuel_depot_password(password: int, minimum_range: int = 0, maximum_range: int = 999999,
                                            strict_repeat: bool = False) -> bool:
    if not isinstance(password, int):
        return False

    if not minimum_range < password < maximum_range:
        return False

    iterable_password = str(password)

    if len(iterable_password) != 6:
        return False

    if not exists_window_of_repeated_digits(iterable_number=iterable_password, window_size=2,
                                            strict_repeat=strict_repeat):
        return False

    if not are_numbers_not_decreasing(iterable_number=iterable_password):
        return False

    return True


def possible_venus_fuel_depot_passwords_in_list(
        minimum_range: int, maximum_range: int, strict_repeat: bool = False,
) -> int:
    return sum(is_a_possible_venus_fuel_depot_password(
        password=password, minimum_range=minimum_range, maximum_range=maximum_range, strict_repeat=strict_repeat,
    ) for password in range(minimum_range, maximum_range + 1))


if __name__ == '__main__':
    minimum_limit, maximum_limit = read_password_range('input.txt')

    '''
    PART 1
    '''
    possible_venus_fuel_depot_passwords = possible_venus_fuel_depot_passwords_in_list(
        minimum_range=minimum_limit, maximum_range=maximum_limit, strict_repeat=False,
    )
    print(possible_venus_fuel_depot_passwords)

    '''
    PART 2
    '''
    possible_venus_fuel_depot_passwords = possible_venus_fuel_depot_passwords_in_list(
        minimum_range=minimum_limit, maximum_range=maximum_limit, strict_repeat=True,
    )
    print(possible_venus_fuel_depot_passwords)
