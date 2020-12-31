@file:Suppress("NOTHING_TO_INLINE")

class Point(val x: Int, val y: Int)

private const val DEFAULT_DEPTH: Short = -1
private const val DEFAULT_QUEUE: Short = 0

class BFS(val width: Int, val height: Int) {
    private val walls = BooleanArray(width * height) { false }

    val cachedPoints = Array(width * height) { Point(it % width, it / width) }

    val depth = ShortArray(width * height) { DEFAULT_DEPTH }
    val queue = ShortArray(width * height) { DEFAULT_QUEUE }

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


    fun path(from: Point, to: Point): Array<Point>? {
        // for optimize use index not Point.
        val fromIndex = from.x + from.y * width
        val toIndex = to.x + to.y * width
        val offsets = shortArrayOf(1, -1, width.toShort(), (-width).toShort())  //performs better as local variable

        // fill use bfs
        depth.setAll(DEFAULT_DEPTH)
        depth[fromIndex] = 0

        queue.setAll(DEFAULT_QUEUE)
        queue[0] = fromIndex.toShort()
        var queueIter = 0
        var queueEnd = 1

        while (queueIter < queueEnd) {
            val index = queue[queueIter].toInt()
            if (index == toIndex) {
                break
            }
            queueIter += 1

            val nLength = (depth[index] + 1).toShort()
            for (offset in offsets) {
                val nIndex = index + offset
                if (depth[nIndex] >= 0 || walls[nIndex]) {
                    continue
                }
                depth[nIndex] = nLength
                queue[queueEnd] = nIndex.toShort()
                queueEnd += 1
            }
        }

        if (queueIter == queueEnd) { // not found
            return null
        }

        // make path

        val pathLength = depth[toIndex] + 1
        val result = Array(pathLength) { to }
        var resultIndex = pathLength - 2

        var index = toIndex
        while (index != fromIndex) {
            val nLength = (depth[index] - 1).toShort()

            for (offset in offsets) {
                val nIndex = index + offset
                if (depth[nIndex] == nLength) {
                    index = nIndex
                    result[resultIndex--] = cachedPoints[index]
                    break // push first found point.
                }
            }
        }

        return result
    }

    inline fun getPoint(x: Int, y: Int): Point {
        return cachedPoints[x + y * width]
    }
}

private inline fun ShortArray.setAll(value: Short) {
    repeat(size) {
        set(it, value)
    }
}
