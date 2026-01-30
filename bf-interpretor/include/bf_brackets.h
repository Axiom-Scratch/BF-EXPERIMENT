#pragma once

#include <cstddef>
#include <string>
#include <unordered_map>

struct BracketMap {
    std::unordered_map<std::size_t, std::size_t> jump;
};

// Builds jump table for '[' and ']'
// Returns false and sets error string on mismatch.
bool build_bracket_map(const std::string& program, BracketMap& map, std::string& error);
