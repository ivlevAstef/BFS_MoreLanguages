//
//  BFS.cpp
//  BFS
//
//  Created by Alexander Ivlev on 30.12.2020.
//

#include <stdio.h>
#include <algorithm>
#include <cstring>
#include "BFS.hpp"

BFS::BFS(int width, int height): width(width), height(height) {
    walls = (bool*)calloc(width * height, sizeof(bool));
    depth = (int16_t*)calloc(width * height, sizeof(int16_t));

    queue = (int16_t*)calloc(width * height, sizeof(int16_t));
}

void BFS::generateWalls() {
    memset(walls, false, width * height * sizeof(bool));

    for (int index = 0; index < width; ++index) {
        walls[index] = true;
        walls[index + (height - 1) * width] = true;
    }
    for (int index = 0; index < height; ++index) {
        walls[index * width] = true;
        walls[width - 1 + index * width] = true;
    }

    int h = height / 10;
    int w = width / 10;
    for (int index = 0; index < height - h; ++index) {
        int x = 2 * w;
        int y = index;
        walls[x + y * width] = true;
    }
    for (int index = h; index < height; ++index) {
        int x = 8 * w;
        int y = index;
        walls[x + y * width] = true;
    }
}
std::vector<Point> BFS::path(Point from, Point to) {
    // for optimize use index not Point.
    const int16_t fromIndex = from.x + from.y * width;
    const int16_t toIndex = to.x + to.y * width;
    const int16_t offsets[4] = { 1, -1, static_cast<int16_t>(width), static_cast<int16_t>(-width) };
    // fill use BFS
    memset(depth, -1, width * height * sizeof(int16_t));

    queue[0] = fromIndex;
    depth[fromIndex] = 0;

    int queueIter = 0;
    int queueEnd = 1;

    while (queueIter < queueEnd) {
        const int16_t index = queue[queueIter];
        if (index == toIndex) {
            break;
        }

        const int32_t& length = depth[index];
        ++queueIter;

        for (const int16_t& offset : offsets) {
            const int16_t nextIndex = index + offset;

            if (depth[nextIndex] >= 0 || walls[nextIndex]) {
                continue;
            }
            depth[nextIndex] = length + 1;

            queue[queueEnd++] = nextIndex;
        }
    }

    if (queueIter == queueEnd) { // not found
        return std::vector<Point>();
    }

    // Make Path
    int16_t index = queue[queueIter];

    std::vector<Point> result;
    result.reserve(depth[index]);

    result.push_back({index % width, index / width});
    while (index != fromIndex) {
        const int32_t& length = depth[index];

        for (const int16_t& offset : offsets) {
            const int16_t nextIndex = index + offset;
            if (depth[nextIndex] == length - 1) {
                index = nextIndex;
                result.push_back({index % width, index / width});
                break; // push first found point.
            }
        }
    }

    std::reverse(std::begin(result), std::end(result));
    return result;

}
