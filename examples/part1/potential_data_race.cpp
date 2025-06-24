#include <thread>
#include <vector>

std::vector<int> data = {1, 2, 3, 4, 5}; // Global data

void worker(int id) {
    // Potential data race - multiple threads accessing shared data
    for (int i = 0; i < data.size(); ++i) {
        data[i] += id; // Race condition!
    }
}

int main() {
    std::thread t1(worker, 1), t2(worker, 2);
    t1.join(); t2.join();
    return 0;
}
