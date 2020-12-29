//
//  BFS.swift
//  BFS
//
//  Created by Alexander Ivlev on 29.12.2020.
//

import Foundation

struct Point: Equatable {
    let x: Int
    let y: Int
}

struct PointInfo {
    let pos: Point
    let length: Int
}

final class BFS {
    private let width: Int
    private let height: Int

    private let walls: UnsafeMutableBufferPointer<Bool>
    private let visited: UnsafeMutableBufferPointer<Bool>
    private let pointsStack: UnsafeMutableBufferPointer<PointInfo>

    init(width: Int, height: Int) {
        self.width = width
        self.height = height
        self.walls = .allocate(capacity: width * height)
        self.visited = .allocate(capacity: width * height)
        self.pointsStack = .allocate(capacity: width * height)
    }

    func generateWalls() {
        walls.assign(repeating: false)
        
        for index in 0..<width {
            walls[index] = true
            walls[index + (height - 1) * width] = true
        }
        for index in 0..<height {
            walls[index * width] = true
            walls[width - 1 + index * width] = true
        }

        let h = height / 10
        let w = width / 10
        for index in 0..<(height - h) {
            let x = 2 * w
            let y = index
            walls[x + y * width] = true
        }
        for index in h..<height {
            let x = 8 * w
            let y = index
            walls[x + y * width] = true
        }
    }

    func path(from: Point, to: Point) -> [Point]? {
        let offsets = [Point(x: 1, y: 0), Point(x: -1, y: 0), Point(x: 0, y: 1), Point(x: 0, y: -1)]

        visited.assign(repeating: false)
        visited[from.x + from.y * width] = true

        pointsStack[0] = PointInfo(pos: from, length: 0)
        var pointsCount = 1
        var index = 0

        while index < pointsCount {
            let info = pointsStack[index]
            if to == info.pos {
                break
            }

            index += 1

            for offset in offsets {
                let p = Point(x: info.pos.x + offset.x, y: info.pos.y + offset.y)
                if visited[p.x + p.y * width] || walls[p.x + p.y * width] {
                    continue
                }
                visited[p.x + p.y * width] = true

                pointsStack[pointsCount] = PointInfo(pos: p, length: info.length + 1)
                pointsCount += 1
            }
        }

        if index >= pointsCount {
            return nil
        }

        var result = ContiguousArray<Point>()
        result.reserveCapacity(pointsStack[index].length)

        result.append(pointsStack[index].pos)
        var currentLength = pointsStack[index].length

        while index > 0 {
            let info = pointsStack[index]
            index -= 1

            if info.length != currentLength - 1 {
                continue
            }

            result.append(info.pos)
            currentLength = info.length
        }

        return result.reversed()
    }
}
