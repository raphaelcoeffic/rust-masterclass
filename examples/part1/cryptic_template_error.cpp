#include <map>
#include <string>

int main() {
    std::map<int, std::string> m;
    m[1] = "hello";
    const auto& cm = m;
    cm[2] = "world"; // Error, but what kind of error?
    return 0;
}

