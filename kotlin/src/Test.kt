
class Test {

    internal fun run() {
        val startTime = System.nanoTime()
        val bfs = BFS(width = 100, height = 100)
        for (index in 0 until 1000) {
            val bfs = BFS(width = 100, height = 100)
            bfs.generateWalls()
            for (subIndex in 0 until 100) {
                val result = bfs.path(Point(0, 0), Point(99, 99))
            }
        }
        val endTime = System.nanoTime()

        val time = (endTime - startTime).toDouble() / 1000000.0
        println("Time: " + time.toString())
    }

    companion object {
        @JvmStatic
        fun main(args: Array<String>) {
            Test().run()
        }
    }
}