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
    private let depth: UnsafeMutableBufferPointer<Int16>
    private let queue: UnsafeMutableBufferPointer<Int>

    init(width: Int, height: Int) {
        self.width = width
        self.height = height
        self.walls = .allocate(capacity: width * height)
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
        // for optimize use index not Point.
        let fromIndex = from.x + from.y * width
        let toIndex = to.x + to.y * width
        let offsets = [1, -1, width, -width]
        // fill use BFS

        depth.assign(repeating: -1)
        depth[fromIndex] = 0

        queue[0] = fromIndex
        var queueIter = 0
        var queueEnd = 1

        while queueIter < queueEnd {
            let index = queue[queueIter]
            if toIndex == index {
                break
            }

            queueIter += 1
            let length = depth[index]

            for offset in offsets {
                let nextIndex = index + offset
                if depth[nextIndex] >= 0 || walls[nextIndex] {
                    continue
                }
                depth[nextIndex] = length + 1

                queue[queueEnd] = nextIndex
                queueEnd += 1
            }
        }

        if queueIter == queueEnd { // not found
            return nil
        }

        // Make path

        var index = queue[queueIter]

        var result = ContiguousArray<Point>()
        result.reserveCapacity(Int(depth[index]))
        result.append(Point(x: index % width, y: index / width))

        while index != fromIndex {
            let length = depth[index]

            for offset in offsets {
                let nextIndex = index + offset
                if depth[nextIndex] == length - 1 {
                    index = nextIndex
                    result.append(Point(x: index % width, y: index / width))
                    break // push first found point.
                }
            }
        }

        return result.reversed()
    }
}
