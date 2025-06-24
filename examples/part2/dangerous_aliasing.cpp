// C++ - References can dangle
int& dangerous_cpp() {
    int local = 42;
    return local; // Dangling reference!
}

void cpp_aliasing() {
    int x = 5;
    int& ref1 = x;
    int& ref2 = x;
    ref1 = 10; // Both ref1 and ref2 see the change
    ref2 = 20; // Potential for confusion
}

int main() {
  auto& dangling = dangerous_cpp();
  cpp_aliasing();
  return 0;
}
