#include <vector>
#include <string>
#include <iostream>

class Resource {
public:
    std::vector<int> data;
    std::string name;
    
    Resource(size_t size, const std::string& n) 
        : data(size, 42), name(n) {
        std::cout << "Created " << name << " with " << size << " elements\n";
    }
    
    // Copy constructor - expensive!
    Resource(const Resource& other) 
        : data(other.data), name(other.name) {
        std::cout << "COPIED " << name << " (expensive!)\n";
    }
    
    // Move constructor - efficient, but optional
    Resource(Resource&& other) noexcept
        : data(std::move(other.data)), name(std::move(other.name)) {
        std::cout << "MOVED " << name << " (efficient!)\n";
    }
};

void cpp_move_semantics() {
    Resource r1(1000000, "BigResource");
    
    // Accidental copy - compiles but slow!
    Resource r2 = r1;  // Copy constructor called
    
    // Explicit move - fast but easy to forget
    Resource r3 = std::move(r1);  // Move constructor called
    
    // r1 is now in "valid but unspecified state" - dangerous!
    // std::cout << r1.name;  // Might work, might not
    
    std::vector<Resource> vec;
    Resource temp(500000, "TempResource");
    
    // Without std::move, this copies!
    vec.push_back(temp);  // COPY - programmer forgot std::move
    
    // With std::move, this moves
    vec.push_back(std::move(temp));  // MOVE - but temp is now unusable
}

int main() {
  cpp_move_semantics();
  return 0;
}
