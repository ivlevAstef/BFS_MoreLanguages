//
//  BFS.cpp
//  BFS
//
//  Created by Alexander Ivlev on 30.12.2020.
//

#include <stdio.h>
#include <algorithm>
#include "BFS.hpp"

static const Point offsets[4] = { {1, 0}, {-1, 0}, {0, 1}, {0, -1} };

BFS::BFS(int width, int height): width(width), height(height) {
    walls = (bool*)calloc(width * height, sizeof(bool));
    visited = (bool*)calloc(width * height, sizeof(bool));
    depth = (int16_t*)calloc(width * height, sizeof(int16_t));

    queue = (Point*)calloc(width * height, sizeof(Point));
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
    // fill use BFS
    memset(visited, false, width * height * sizeof(bool));
    memset(depth, -1, width * height * sizeof(int16_t));

    visited[from.x + from.y * width] = true;
    depth[from.x + from.y * width] = 0;

    queue[0] = from;
    int queueIter = 0;
    int queueEnd = 1;

    while (queueIter < queueEnd) {
        const Point& pos = queue[queueIter];
        const int32_t& length = depth[pos.x + pos.y * width];
        if (pos.x == to.x && pos.y == to.y) {
            break;
        }

        ++queueIter;

        for (const Point& offset : offsets) {
            const int x = pos.x + offset.x;
            const int y = pos.y + offset.y;

            if (visited[x + y * width] || walls[x + y * width]) {
                continue;
            }
            visited[x + y * width] = true;

            queue[queueEnd++] = {x,y};
            depth[x + y * width] = length + 1;
        }
    }

    if (queueIter == queueEnd) { // not found
        return std::vector<Point>();
    }

    // Make Path
    Point pos = queue[queueIter];

    std::vector<Point> result;
    result.reserve(depth[pos.x + pos.y * width]);

    result.push_back(pos);
    while (pos.x != from.x || pos.y != from.y) {
        const int32_t& length = depth[pos.x + pos.y * width];

        for (const Point& offset : offsets) {
            const int x = pos.x + offset.x;
            const int y = pos.y + offset.y;
            if (depth[x + y * width] == length - 1) {
                pos.x = x;
                pos.y = y;
                result.push_back({x, y});
                break; // push first found point.
            }
        }
    }

    std::reverse(std::begin(result), std::end(result));
    return result;

}
