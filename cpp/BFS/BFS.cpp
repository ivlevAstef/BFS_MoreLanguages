//
//  BFS.cpp
//  BFS
//
//  Created by Alexander Ivlev on 30.12.2020.
//

#include <stdio.h>
#include <algorithm>
#include "BFS.hpp"

struct PointInfo {
    Point pos;
    int length;
};

static const Point offsets[4] = { {1, 0}, {-1, 0}, {0, 1}, {0, -1} };

BFS::BFS(int width, int height): width(width), height(height) {
    walls = (bool*)calloc(width * height, sizeof(bool));
    visited = (bool*)calloc(width * height, sizeof(bool));
}

void BFS::generateWalls() {
    memset(walls, 0, width * height * sizeof(bool));

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
    memset(visited, 0, width * height * sizeof(bool));

    std::vector<PointInfo> points;
    points.reserve(width * height);

    visited[from.x + from.y * width] = true;
    points.push_back({ from, 0 });

    int index = 0;
    while (index < points.size()) {
        PointInfo info = points[index];
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

            points.push_back({ p, info.length + 1 });
        }
    }

    if (index >= points.size()) {
        return std::vector<Point>();
    }

    std::vector<Point> result;
    result.reserve(points[index].length);
    result.push_back(points[index].pos);

    int currentLength = points[index].length;

    while (index > 0) {
        PointInfo info = points[index];
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
