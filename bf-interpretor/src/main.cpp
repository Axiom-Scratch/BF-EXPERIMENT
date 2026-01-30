#include "bf_vm.h"
#include "bf_brackets.h"

#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

static std::string read_all(const std::string& path) {
    std::ifstream f(path, std::ios::binary);
    if (!f) return {};
    std::ostringstream ss;
    ss << f.rdbuf();
    return ss.str();
}

int main(int argc, char** argv) {
    if (argc < 2) {
        std::cerr << "Usage: bf <program.bf>\n";
        return 1;
    }

    const std::string program = read_all(argv[1]);
    if (program.empty()) {
        std::cerr << "Failed to read program (or empty): " << argv[1] << "\n";
        return 1;
    }

    BracketMap bm;
    std::string err;
    if (!build_bracket_map(program, bm, err)) {
        std::cerr << "Bracket error: " << err << "\n";
        return 1;
    }

    BFVM vm;
    if (!vm.run(program, bm, std::cin, std::cout)) {
        std::cerr << "\nRuntime error: pointer out of bounds.\n";
        return 2;
    }

    return 0;
}
