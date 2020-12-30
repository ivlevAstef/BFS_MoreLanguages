//
//  main.swift
//  BFS
//
//  Created by Alexander Ivlev on 29.12.2020.
//

import Foundation

func runBfs() {
    let startTime = DispatchTime.now()
    let bfs = BFS(width: 100, height: 100)
    bfs.generateWalls()

    for _ in 0..<100000 {
        _ = bfs.path(from: Point(x: 1, y: 1), to: Point(x: 98, y: 98))
    }
    let endTime = DispatchTime.now()

    let time = Double(endTime.uptimeNanoseconds - startTime.uptimeNanoseconds) / 1000000.0
    print("Time: \(time)")
}


runBfs()
