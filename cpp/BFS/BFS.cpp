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

    pointsStack = (PointInfo*)calloc(width * height, sizeof(PointInfo));
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
    memset(visited, false, width * height * sizeof(bool));

    visited[from.x + from.y * width] = true;
    pointsStack[0] = { from, 0 };

    int index = 0;
    int pointsCount = 1;
    while (index < pointsCount) {
        PointInfo info = pointsStack[index];
        if (info.pos.x == to.x && info.pos.y == to.y) {
            break;
        }

        ++index;

        for (const Point& offset : offsets) {
            Point p = { info.pos.x + offset.x, info.pos.y + offset.y };

            if (visited[p.x + p.y * width] || walls[p.x + p.y * width]) {
                continue;
            }
            visited[p.x + p.y * width] = true;

            pointsStack[pointsCount] = { p, info.length + 1 };
            ++pointsCount;
        }
    }

    if (index >= pointsCount) {
        return std::vector<Point>();
    }

    std::vector<Point> result;
    result.reserve(pointsStack[index].length);
    result.push_back(pointsStack[index].pos);

    int currentLength = pointsStack[index].length;

    while (index > 0) {
        PointInfo info = pointsStack[index];
        --index;

        if (info.length != currentLength - 1) {
            continue;
        }

        result.push_back(info.pos);
        currentLength = info.length;
    }

    std::reverse(std::begin(result), std::end(result));
    return result;

}
