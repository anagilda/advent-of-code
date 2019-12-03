import itertools
from copy import deepcopy
from typing import (
    List,
    Optional,
    Tuple,
)


def read_gravity_assist_program(path: str) -> List[int]:
    with open(path) as input_file:
        return [int(intcode) for intcode in input_file.read().split(',')]


def intcode_computer(program: List[int]) -> List[int]:
    instruction_pointer = 0

    while opcode := program[instruction_pointer]:
        if opcode == 99:
            return program

        input_noun_address = program[instruction_pointer + 1]
        input_verb_address = program[instruction_pointer + 2]
        output_address = program[instruction_pointer + 3]

        if opcode == 1:
            program[output_address] = program[input_noun_address] + program[input_verb_address]

        elif opcode == 2:
            program[output_address] = program[input_noun_address] * program[input_verb_address]

        instruction_pointer += 4

    return program


def restore_program_noun_and_verb(program: List[int], noun: int, verb: int) -> List[int]:
    program[1] = noun
    program[2] = verb

    return program


def restore_program_to_1202_program_alarm(program: List[int]) -> List[int]:
    return restore_program_noun_and_verb(program=program, noun=12, verb=2)


def complete_gravity_assist(program: List[int], output: int) -> Tuple[Optional[int], Optional[int]]:
    for noun, verb in itertools.product(range(0, 99), repeat=2):
        test_program = deepcopy(program)
        test_program = restore_program_noun_and_verb(program=test_program, noun=noun, verb=verb)
        completed_program_output = intcode_computer(program=test_program)
        if completed_program_output[0] == output:
            return 100 * noun + verb

    return None, None


if __name__ == '__main__':
    '''
    Part 1
    '''
    gravity_assist_program = read_gravity_assist_program('input.txt')

    gravity_assist_program_alarm = restore_program_to_1202_program_alarm(gravity_assist_program)
    completed_program = intcode_computer(gravity_assist_program)

    print(f'Position 0 when program halts is: {completed_program[0]}')  # 4330636

    '''
    Part 2
    '''
    gravity_assist_program = read_gravity_assist_program('input.txt')
    gravity_assist_output = 19690720

    gravity_assist = complete_gravity_assist(program=gravity_assist_program, output=gravity_assist_output)

    print(gravity_assist)  # 6086
