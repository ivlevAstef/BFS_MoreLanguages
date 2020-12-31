//
//  main.cpp
//  BFS
//
//  Created by Alexander Ivlev on 29.12.2020.
//

#include <chrono>
#include <iostream>
#include "BFS.hpp"

void test() {
    auto start = std::chrono::high_resolution_clock::now();
    BFS bfs = BFS(100, 100);
    bfs.generateWalls();

    for(int index = 0; index < 100000; ++index) {
        auto result = bfs.path({1,1}, {98,98});
    }
    auto end = std::chrono::high_resolution_clock::now();

    auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);

    std::cout << "Time: " << elapsed.count() << std::endl;
}

int main(int argc, const char * argv[]) {
    test();
    return 0;
}
