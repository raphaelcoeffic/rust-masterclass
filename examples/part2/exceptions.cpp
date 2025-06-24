#include <fstream>
#include <iostream>
#include <stdexcept>

std::string read_file(const std::string &filename) {
    std::ifstream file(filename);
    if (!file.is_open()) {
        throw std::runtime_error("Could not open file: " + filename);
    }

    std::string content, line;
    while (std::getline(file, line)) {
        content += line + "\n";
    }

    if (content.empty()) {
        throw std::runtime_error("File is empty");
    }

    return content;
}

void process() {
    try {
        auto content = read_file("data.txt");
        std::cout << "File content: " << content << std::endl;
    } catch (const std::exception &e) {
        std::cerr << "Error: " << e.what() << std::endl;
        // What specific error occurred? Hard to tell.
    }
}

int main() {
  process();
  return 0;
}
