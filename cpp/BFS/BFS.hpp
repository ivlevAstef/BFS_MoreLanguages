//
//  BFS.hpp
//  BFS
//
//  Created by Alexander Ivlev on 29.12.2020.
//

#ifndef BFS_h
#define BFS_h

#include <vector>

struct Point {
    int x;
    int y;
};

struct PointInfo {
    Point pos;
    int length;
};

class BFS {
public:
    BFS(int width, int height);

    void generateWalls();
    std::vector<Point> path(Point from, Point to);

private:
    int width;
    int height;
    
    bool* walls;
    bool* visited;

    PointInfo* pointsStack;
};


#endif /* BFS_h */
