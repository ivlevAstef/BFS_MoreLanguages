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
        val toX = to.x
        val toY = to.y
        val offsets = arrayOf(Point(1, 0), Point(-1, 0), Point(0, 1), Point(0, -1))

        val offsetsX = arrayOf(1, -1, 0, 0);
        val offsetsY = arrayOf(0, 0, 1, -1);

        for (index in 0 until width * height) {
            visited[index] = false
        }
        visited[from.x + from.y * width] = true

        val pointsX = mutableListOf(from.x)
        val pointsY = mutableListOf(from.y)
        val pointsL = mutableListOf(0)
        var index = 0

        while (index < pointsX.count()) {
            val posX = pointsX[index]
            val posY = pointsY[index]
            val length = pointsL[index]

            if (toX == posX && toY == posY) {
                break
            }

            index += 1

            for (oindex in 0..3) {
                val nX = posX + offsetsX[oindex]
                val nY = posY + offsetsY[oindex]
                if (visited[nX + nY * width] || walls[nX + nY * width]) {
                    continue
                }
                visited[nX + nY * width] = true

                pointsX.add(nX)
                pointsY.add(nY)
                pointsL.add(length + 1)
            }
        }

        if (index >= pointsX.count()) {
            return null
        }

        val result = mutableListOf<Point>()

        result.add(Point(pointsX[index], pointsY[index]))
        var currentLength = pointsL[index]

        while (index > 0) {
            index -= 1

            if (pointsL[index] != currentLength - 1) {
                continue
            }

            result.add(Point(pointsX[index], pointsY[index]))
            currentLength = pointsL[index]
        }

        return result.reversed()
    }
}