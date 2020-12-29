import kotlin.random.Random

data class Point(val x: Int, val y: Int) {
    override fun equals(other: Any?): Boolean {
        return other?.let { obj ->
            if (obj is Point) {
                return obj.x == this.x && obj.y == this.y
            }
            return false
        } ?: false
    }
}

class BFS(val width: Int, val height: Int) {
    private val walls: Array<Boolean> = Array(width * height, { false })
    private val visited: Array<Boolean> = Array(width * height, { false })

    fun generateWalls() {
        for (index in 0 until width * height) {
            walls[index] = false
        }

        for (index in 0 until width * height / 10) {
            val x = Random.nextInt(0, width)
            val y = Random.nextInt(0, height)
            walls[x + y * width] = true
        }
    }

    fun path(from: Point, to: Point): List<Point>? {
        val offsets = arrayOf(Point(1, 0), Point(-1, 0), Point(0, 1), Point(0, -1))

        for (index in 0 until width * height) {
            visited[index] = false
        }
        visited[from.x + from.y * width] = true

        val points = mutableListOf(Pair(from, 0))
        var index = 0

        while (index < points.count()) {
            val pair = points[index]
            val pos = pair.first
            val length = pair.second

            if (to == pos) {
                break
            }

            index += 1

            for (offset in offsets) {
                val nX = pos.x + offset.x
                val nY = pos.y + offset.y
                if (nX < 0 || nX >= width || nY < 0 || nY >= height || visited[nX + nY * width]) {
                    continue
                }
                visited[nX + nY * width] = true

                if (walls[nX + nY * width]) {
                    continue
                }

                points.add(Pair(Point(nX, nY), length + 1))
            }
        }

        if (index >= points.count()) {
            return null
        }

        val result = mutableListOf<Point>()

        result.add(points[index].first)
        var currentLength = points[index].second

        while (index > 0) {
            val pair = points[index]
            index -= 1

            if (pair.second != currentLength - 1) {
                continue
            }

            result.add(pair.first)
            currentLength = pair.second
        }

        return result.reversed()
    }
}