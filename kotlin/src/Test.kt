
class Test {

    internal fun run() {
        val bfsAccelerate = BFS(width = 100, height = 100)
        bfsAccelerate.generateWalls()
        for (index in 0 until 10000) {
            val result = bfsAccelerate.path(Point(1, 1), Point(98, 98))
        }

        Thread.sleep(5000L)

        val startTime = System.nanoTime()
        val bfs = BFS(width = 100, height = 100)
        bfs.generateWalls()
        for (index in 0 until 100000) {
            val result = bfs.path(Point(1, 1), Point(98, 98))
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