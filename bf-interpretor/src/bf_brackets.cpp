#include "bf_brackets.h"
#include <stack>

bool build_bracket_map(const std::string& program, BracketMap& map, std::string& error) {
    std::stack<std::size_t> st;
    map.jump.clear();

    for (std::size_t i = 0; i < program.size(); i++) {
        char c = program[i];
        if (c == '[') {
            st.push(i);
        } else if (c == ']') {
            if (st.empty()) {
                error = "Unmatched ']' at position " + std::to_string(i);
                return false;
            }
            std::size_t open = st.top();
            st.pop();
            map.jump[open] = i;
            map.jump[i] = open;
        }
    }

    if (!st.empty()) {
        error = "Unmatched '[' at position " + std::to_string(st.top());
        return false;
    }

    return true;
}
