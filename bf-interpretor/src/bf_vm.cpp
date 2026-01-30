#include "bf_vm.h"

#include <algorithm> // std::fill

BFVM::BFVM(BFConfig cfg) : cfg_(cfg), tape_(cfg_.tape_size, 0), dp_(0) {}

void BFVM::reset() {
    std::fill(tape_.begin(), tape_.end(), 0);
    dp_ = 0;
}

bool BFVM::move_right() {
    if (dp_ + 1 >= tape_.size()) {
        if (cfg_.strict_pointer) return false;
        dp_ = 0;
        return true;
    }
    dp_++;
    return true;
}

bool BFVM::move_left() {
    if (dp_ == 0) {
        if (cfg_.strict_pointer) return false;
        dp_ = tape_.size() - 1;
        return true;
    }
    dp_--;
    return true;
}

bool BFVM::run(const std::string& program,
               const BracketMap& brackets,
               std::istream& in,
               std::ostream& out)
{
    for (std::size_t ip = 0; ip < program.size(); ip++) {
        const char c = program[ip];

        switch (c) {
            case '>':
                if (!move_right()) return false;
                break;

            case '<':
                if (!move_left()) return false;
                break;

            case '+':
                tape_[dp_] = static_cast<std::uint8_t>(tape_[dp_] + 1);
                break;

            case '-':
                tape_[dp_] = static_cast<std::uint8_t>(tape_[dp_] - 1);
                break;

            case '.':
                out.put(static_cast<char>(tape_[dp_]));
                break;

            case ',':
            {
                int ch = in.get();
                tape_[dp_] = (ch == EOF) ? 0 : static_cast<std::uint8_t>(ch);
                break;
            }

            case '[':
                // if current cell is zero, jump to matching ']'
                if (tape_[dp_] == 0) {
                    ip = brackets.jump.at(ip); // points to ']'
                }
                break;

            case ']':
                // if current cell is nonzero, jump back to matching '['
                if (tape_[dp_] != 0) {
                    ip = brackets.jump.at(ip); // points to '['
                }
                break;

            default:
                // ignore non-BF characters
                break;
        }
    }

    return true;
}
