import re
from pathlib import Path
from iterage import itr, fn


def is_valid1(i):
    number = range(int(i[0]), int(i[1]) + 1)
    char: str = i[2]
    pwd: str = i[3]

    return pwd.count(char) in number


def calc1(input: str):
    return itr(input.split("\n")) \
        .map(lambda line: re.fullmatch(r"(\d+)-(\d+)\s+(.):\s+(.+)", line)) \
        .where(lambda match: match is not None) \
        .map(lambda match: match.groups()) \
        .where(is_valid1) \
        .len()


def is_valid2(i):
    number1 = int(i[0]) - 1
    number2 = int(i[1]) - 1
    char: str = i[2]
    pwd: str = i[3]

    return (pwd[number1] == char) != (pwd[number2] == char)


def calc2(input: str):
    return itr(input.split("\n")) \
        .map(lambda line: re.fullmatch(r"(\d+)-(\d+)\s+(.):\s+(.+)", line)) \
        .where(lambda match: match is not None) \
        .map(lambda match: match.groups()) \
        .where(is_valid2) \
        .len()


def test_1():
    assert calc1("""
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc""") == 2


def test_2():
    assert calc2("""
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc""") == 1


if __name__ == '__main__':
    test_1()
    test_2()

    print(calc1(Path("aoc02.txt").read_text()))
    print(calc2(Path("aoc02.txt").read_text()))
