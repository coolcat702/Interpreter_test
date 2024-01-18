from sys import argv

def print_state(state: list[bool], pos: int, time: int) -> None:
    res: str = ""
    for i in range(len(state)):
        res += str(int(state[i]))
    print(f"\nOutput: {res}\n")
    print(f"Cursor at: {pos}")
    print(f"Took {time} iterations")

def remove_excess(input_string: str) -> str:
    res: str = ""
    found: bool = False

    for char in input_string:
        if char.isdigit():
            res += char
            found = True
        elif found:
            break
        else:
            res += char

    return res

def debug(argv: list[str]) -> None:
    print("Program: ")
    errors: int = 0
    paths: dict[str, int] = {}
    states: list[str] = [line.split("//")[0].strip() for line in open(argv[1], "r") if line]
    for state in states:
        print(state)
    print()
    for idx, state in enumerate(states):
        for path in state.split(" "):
            paths.update({path: idx})
    visited_states: set[int] = set()
    jumped_states: set[int] = set()
    for path in paths.keys():
        for char in path:
            if char in "0123456789<>!{}/\\":
                continue
            print(f"Invalid character {char} in line {paths[path]}")
            errors += 1
        if remove_excess(path) != path:
            print(f"Unreachable characters {path - remove_excess(path)} in line {paths[path]}. Consider removing them")
            errors += 1
        visited_states.add(paths[path]+1)
        jumped_states.add(int("".join([x if x in "0123456789" else "" for x in path])))
    
    all_states: set[int] = set(range(1, len(states)+1))
    unused_states: set[int] = all_states - visited_states
    invalid_states: set[int] = jumped_states - all_states
    print(f"States: {''.join(f'{state}, ' for state in all_states).strip(', ')}")

    for unused_state in unused_states:
        print(f"Unused state {unused_state+1}")
        errors += 1
    for invalid_state in invalid_states:
        print(f"Invalid jump to state {invalid_state}")
        errors += 1
    print(f"Debugging finished with {errors} errors found.\n")

def run_machine(tape: list[bool], states: list[str], debug: bool) -> tuple[list[bool], int]:
    pos: int = 0
    idx: int = 1
    cur: list[bool] = tape
    time: int = 0
    while states[idx-1] != "END":
        if pos < 0:
            pos += 1
            break
        if pos >= len(cur):
            pos -= 1
            break
        time += 1
        val: bool = cur[pos]
        state: list[str] = states[idx-1].split(" ")
        if len(state) == 1:
            path = state[0]
        else:
            path = state[val]
        jump: int = 0
        if debug:
            print(f"State: {idx} On path {int(val)} ({path}) at {pos}")
        for char in path:
            if char in "0123456789":
                jump *= 10
                jump += int(char)

        for char in path:
            if char.isdigit():
                idx = jump
                break
            elif char == ">":
                pos += 1
            elif char == "<":
                pos -= 1
            elif char == "!":
                cur[pos] = not cur[pos]
            elif char == "{":
                pos = 0
            elif char == "}":
                pos = len(cur) - 1
            elif char == "/":
                cur[pos] = False
            elif char == "\\":
                cur[pos] = True

    return cur, pos, time

def main(argc: int, argv: list[str]) -> None:
    if argc != 2:
        print("Usage: python trmc.py <filename.trmc>")
        return
    do_debug: bool = True
    print()
    if input(f"Run in Debug Mode? ").lower().strip() in ("n", "no"):
        do_debug = False
    print()
    if do_debug:
        debug(argv)
    tape: list[bool] = [bool(int(x)) if x in "01" else None for x in input("Input: ").strip()]
    print()
    if None in tape:
        print(f"Invalid input, must be binary integer")
        return
    states: list[str] = [line.split("//")[0].strip() for line in open(argv[1], "r") if line]
    pos: int = 0
    time: int = 0
    tape, pos, time = run_machine(tape, states, do_debug)
    print_state(tape, pos, time)
    print()

if __name__ == "__main__":
    main(len(argv), argv)
