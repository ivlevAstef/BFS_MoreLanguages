
class Test {
    private fun warmUp() {
        val bfs = BFS(width = 100, height = 100)
        bfs.generateWalls()
        for (index in 0 until 10000) {
            val result = bfs.path(bfs.getPoint(1, 1), bfs.getPoint(98, 98))
        }
        Thread.sleep(2_000L)
    }

    internal fun run() {
        //warmUp()

        val startTime = System.nanoTime()
        val bfs = BFS(width = 100, height = 100)
        bfs.generateWalls()
        for (index in 0 until 100000) {
            val result = bfs.path(bfs.getPoint(1, 1), bfs.getPoint(98, 98))
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