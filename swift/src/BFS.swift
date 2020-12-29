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
    let width: Int
    let height: Int

    private var walls: [Bool] = []

    init(width: Int, height: Int) {
        self.width = width
        self.height = height
    }

    func generateWalls() {
        var newWalls = [Bool](repeating: false, count: width * height)
        for _ in 0..<(width * height / 10) {
            let x = Int.random(in: 0..<width)
            let y = Int.random(in: 0..<height)
            newWalls[x + y * width] = true
        }

        walls = newWalls
    }

    func path(from: Point, to: Point) -> [Point]? {
        let offsets = [Point(x: 1, y: 0), Point(x: -1, y: 0), Point(x: 0, y: 1), Point(x: 0, y: -1)]

        var visited = [Bool](repeating: false, count: width * height)
        visited[from.x + from.y * width] = true

        var points: [PointInfo] = [PointInfo(pos: from, length: 0)]
        var index = 0

        while index < points.count {
            let info = points[index]
            if to == info.pos {
                break
            }

            index += 1

            for offset in offsets {
                let p = Point(x: info.pos.x + offset.x, y: info.pos.y + offset.y)
                if p.x < 0 || p.x >= width || p.y < 0 || p.y >= height || visited[p.x + p.y * width] {
                    continue
                }
                visited[p.x + p.y * width] = true

                if walls[p.x + p.y * width] {
                    continue
                }

                points.append(PointInfo(pos: p, length: info.length + 1))
            }
        }

        if index >= points.count {
            return nil
        }

        var result: [Point] = []
        result.append(points[index].pos)
        var currentLength = points[index].length

        while index > 0 {
            let info = points[index]
            index -= 1

            if info.length != currentLength - 1 {
                continue
            }

            result.insert(info.pos, at: 0)
            currentLength = info.length
        }

        return result
    }
}
