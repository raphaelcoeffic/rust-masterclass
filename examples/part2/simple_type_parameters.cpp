#include <string>
#include <vector>

template<typename T>
class Container {
private:
    std::vector<T> data;
public:
    void push(const T& item) { data.push_back(item); }
    T& get(size_t index) { return data[index]; }
    size_t size() const { return data.size(); }
};

template<typename T>
T max_value(const T& a, const T& b) {
    return (a > b) ? a : b;
}

void cpp_generics_example() {
    Container<int> int_container;
    Container<std::string> string_container;

    int_container.push(42);
    string_container.push("hello");

    // Type deduction works here
    auto result1 = max_value(10, 20);
    auto result2 = max_value(3.14, 2.71);

    // But explicit types sometimes needed
    auto result3 = max_value<double>(10, 3.14);  // Mixed types
}

int main() {
  cpp_generics_example();
  return 0;
}
