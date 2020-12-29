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
    for _ in 0..<1000 {
        bfs.generateWalls()
        for _ in 0..<100 {
            let result = bfs.path(from: Point(x: 0, y: 0), to: Point(x: 99, y: 99))
            _ = result
        }
    }
    let endTime = DispatchTime.now()

    let time = Double(endTime.uptimeNanoseconds - startTime.uptimeNanoseconds) / 1000000.0
    print("Time: \(time)")
}


runBfs()
