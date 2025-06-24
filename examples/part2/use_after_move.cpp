#include <iostream>
#include <string>

void cpp_use_after_move_bug() {
    std::string s1 = "Hello";
    std::string s2 = std::move(s1);

    // This compiles but is undefined behaviour!
    std::cout << s1 << std::endl;  // What gets printed?
    s1.append(" World");           // Might crash, might work

    // std::move doesn't actually move - it just casts to rvalue reference
    // The actual move happens in the constructor/assignment
}

int main() {
  cpp_use_after_move_bug();
  return 0;
}
