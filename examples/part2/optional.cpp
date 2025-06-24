// MUST be compiled with --std==c++17
#include <optional>
#include <iostream>

// std::optional - built-in, but limited
void cpp_optional_example() {
    std::optional<int> maybe_value = 42;
    std::optional<int> no_value;

    if (maybe_value.has_value()) {
        std::cout << "Value: " << maybe_value.value() << std::endl;
    }

    if (!no_value.has_value()) {
        std::cout << "No value" << std::endl;
    }
}

int main() {
  cpp_optional_example();
}
