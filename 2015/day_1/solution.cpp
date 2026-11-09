#include <fstream>
#include <iostream>
#include <vector>

int floor(std::vector<char> &instructions) {
    int floor = 0;
    for (auto c : instructions) {
        if (c == '(') {
            floor += 1;
        } else {
            floor -= 1;
        }
    }

    return floor;
}

int position(std::vector<char> &instructions) {
    int floor = 0;
    for (int i = 0; i < instructions.size(); i += 1) {
        if (instructions[i] == '(') {
            floor += 1;
        } else {
            floor -= 1;
        }
        if (floor == -1) {
            return i + 1;
        }
    }
}

int main() {
    std::ifstream input("input");

    std::vector<char> instructions((std::istreambuf_iterator<char>(input)),
                                   std::istreambuf_iterator<char>());

    std::cout << position(instructions) << std::endl;
}
