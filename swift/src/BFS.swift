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

final class BFS {
    private let width: Int
    private let height: Int

    private let walls: UnsafeMutableBufferPointer<Bool>
    private let visited: UnsafeMutableBufferPointer<Bool>
    private let depth: UnsafeMutableBufferPointer<Int16>
    private let queue: UnsafeMutableBufferPointer<Point>

    init(width: Int, height: Int) {
        self.width = width
        self.height = height
        self.walls = .allocate(capacity: width * height)
        self.visited = .allocate(capacity: width * height)
        self.depth = .allocate(capacity: width * height)
        self.queue = .allocate(capacity: width * height)
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
        // fill use BFS

        depth.assign(repeating: -1)
        depth[from.x + from.y * width] = 0

        visited.assign(repeating: false)
        visited[from.x + from.y * width] = true

        queue[0] = from
        var queueIter = 0
        var queueEnd = 1

        while queueIter < queueEnd {
            let pos = queue[queueIter]
            let length = depth[pos.x + pos.y * width]
            if to == pos {
                break
            }

            queueIter += 1

            for offset in offsets {
                let p = Point(x: pos.x + offset.x, y: pos.y + offset.y)
                if visited[p.x + p.y * width] || walls[p.x + p.y * width] {
                    continue
                }
                visited[p.x + p.y * width] = true

                queue[queueEnd] = p
                queueEnd += 1
                depth[p.x + p.y * width] = length + 1
            }
        }

        if queueIter == queueEnd { // not found
            return nil
        }

        // Make path

        var pos = queue[queueIter]

        var result = ContiguousArray<Point>()
        result.reserveCapacity(Int(depth[pos.x + pos.y * width]))
        result.append(pos)

        while pos != from {
            let length = depth[pos.x + pos.y * width]

            for offset in offsets {
                let p = Point(x: pos.x + offset.x, y: pos.y + offset.y)
                if depth[p.x + p.y * width] == length - 1 {
                    pos = p
                    result.append(p)
                    break // push first found point.
                }
            }
        }

        return result.reversed()
    }
}
