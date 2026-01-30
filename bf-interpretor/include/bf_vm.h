#pragma once

#include <cstdint>
#include <cstddef>
#include <string>
#include <vector>
#include <istream>
#include <ostream>

#include "bf_brackets.h"

struct BFConfig {
    std::size_t tape_size = 30000;
    bool strict_pointer = true; // if false, dp wraps around
};

class BFVM {
public:
    explicit BFVM(BFConfig cfg = {});
    void reset();

    // Stage 2: full interpreter with loops using prebuilt bracket map
    bool run(const std::string& program,
             const BracketMap& brackets,
             std::istream& in,
             std::ostream& out);

private:
    BFConfig cfg_;
    std::vector<std::uint8_t> tape_;
    std::size_t dp_ = 0;

    bool move_right();
    bool move_left();
};
