
import scala.collection.mutable.HashSet
import scala.collection.mutable.Map
import scala.math.max

@main def part2(): Int = {
  var ans: Int = 0

  var lines = scala.io.Source.fromFile("../prod2.txt").getLines().toArray

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

  val maxDim = max(nRows, nCols)

  for ((_, coords) <- symbols) {
    val coordsLen = coords.length
    for (i <- 0 until coordsLen) {
      for (j <- (i+1) until coordsLen) {

        val (x1, y1) = coords(i)
        val (x2, y2) = coords(j)
        val deltaX = x2 - x1
        val deltaY = y2 - y1

        // Now, need to calculate for more positions
        // NOTE: remember that Array are fixed-sized, so either use an ArrayBuffer, or replace the original one (or create another one)
        //  when trying to modify it inplace
        var totalAntinodes = Array[(Int, Int)]()
        for (constant <- 0 until maxDim) {
          val leftAntinode = (x1 - deltaX * constant, y1 - deltaY * constant)
          val rightAntinode = (x2 + deltaX * constant, y2 + deltaY * constant)
          totalAntinodes = totalAntinodes ++ Array(leftAntinode, rightAntinode)
        }
        for (node <- totalAntinodes) {
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
