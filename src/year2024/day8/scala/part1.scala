
import scala.collection.mutable.HashSet
import scala.collection.mutable.Map

@main def part1(): Int = {
  var ans: Int = 0

  var lines = scala.io.Source.fromFile("../prod1.txt").getLines().toArray

  var grid = lines.map(_.split("").toArray)
  var symbols = Map[String, List[(Int, Int)]]()

  val nRows = grid.length
  val nCols = grid(0).length

  val antiNodes = HashSet[(Int, Int)]()

  // Define symbols and their coords
  for (i <- 0 until nRows) {
    for (j <- 0 until nCols) {
      if (grid(i)(j) != ".") {
        symbols(grid(i)(j)) = symbols.getOrElse(grid(i)(j), List()) :+ (i, j)
      }
    }
  }

  for ((_, coords) <- symbols) {
    val coordsLen = coords.length
    for (i <- 0 until coordsLen) {
      for (j <- (i+1) until coordsLen) {

        val (x1, y1) = coords(i)
        val (x2, y2) = coords(j)
        val deltaX = x2 - x1
        val deltaY = y2 - y1

        val leftAntinode = (x1 - deltaX, y1 - deltaY)
        val rightAntinode = (x2 + deltaX, y2 + deltaY)

        for (node <- List(leftAntinode, rightAntinode)) {
          val (antiX, antiY) = node
          if (antiX >= 0 && antiX < nRows && antiY >= 0 && antiY < nCols) {
            grid(antiX)(antiY) = "#"
            antiNodes += node
          }

        }
      }
    }
  }

  antiNodes.size
}
