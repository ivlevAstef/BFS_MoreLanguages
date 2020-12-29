data class Point(val x: Int, val y: Int)

class BFS(val width: Int, val height: Int) {
    private val walls: Array<Boolean> = Array(width * height, { false })
    private val visited: Array<Boolean> = Array(width * height, { false })
    private val depth: Array<Int> = Array(width * height, { 0 })

    fun generateWalls() {
        for (index in 0 until width * height) {
            walls[index] = false
        }
        for (index in 0 until width) {
            walls[index] = true
            walls[index + (height - 1) * width] = true
        }
        for (index in 0 until height) {
            walls[index * width] = true
            walls[width - 1 + index * width] = true
        }

        val h = height / 10
        val w = width / 10
        for (index in 0 until height - h) {
            val x = 2 * w
            val y = index
            walls[x + y * width] = true
        }
        for (index in h until height) {
            val x = 8 * w
            val y = index
            walls[x + y * width] = true
        }
    }

    fun path(from: Point, to: Point): List<Point>? {
        val offsets = arrayOf(Point(1, 0), Point(-1, 0), Point(0, 1), Point(0, -1))
        // fill use bfs

        for (index in 0 until width * height) {
            visited[index] = false
            depth[index] = -1
        }
        visited[from.x + from.y * width] = true
        depth[from.x + from.y * width] = 0

        val queue = mutableListOf(from)
        var queueIter = 0

        while (queueIter < queue.count()) {
            val pos = queue[queueIter]

            if (to.x == pos.x && to.y == pos.y) {
                break
            }

            queueIter += 1

            val length = depth[pos.x + pos.y * width]
            for (offset in offsets) {
                val nX = pos.x + offset.x
                val nY = pos.y + offset.y
                if (visited[nX + nY * width] || walls[nX + nY * width]) {
                    continue
                }
                visited[nX + nY * width] = true

                queue.add(Point(nX, nY))
                depth[nX + nY * width] = length + 1
            }
        }

        if (queueIter == queue.count()) { // not found
            return null
        }

        // make path

        val result = mutableListOf<Point>()
        result.add(to)

        var pos = result.last()
        while (pos.x != from.x || pos.y != from.y) {
            val length = depth[pos.x + pos.y * width]

            for (offset in offsets) {
                val nX = pos.x + offset.x
                val nY = pos.y + offset.y
                if (depth[nX + nY * width] == length - 1) {
                    pos = Point(nX, nY)
                    result.add(pos)
                    break // push first found point.
                }
            }
        }

        return result.reversed()
    }
}