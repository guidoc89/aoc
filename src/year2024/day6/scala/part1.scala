import scala.util.control.Breaks._
import scala.collection.mutable.HashSet

@main def part1(): Int = {
  var lines = scala.io.Source.fromFile("../prod1.txt").getLines().toArray
  var grid = lines.map(_.split("").toArray)
  var ans: Int = 0

  // NOTE: remember this syntax
  var visited =  HashSet[(Int, Int)]()
  var cycle = 0
  var nRows = grid.length
  var nCols = grid(0).length
  var guardPosition = (0, 0)

  breakable {
    for (i <- 0 until nRows) {
      for (j <- 0 until nCols) {
        if (grid(i)(j) == "^") {
          guardPosition = (i, j)
          break
        }
      }
    }
  }

  var DIRECTIONS = List(
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1),
    )
  visited += guardPosition

  breakable {
    for (i <- 0 until nRows) {
      for (j <- 0 until nCols) {
        var (deltaR, deltaC) = DIRECTIONS(cycle)
        var newR = guardPosition(0) + deltaR
        var newC = guardPosition(1) + deltaC

        if (newR >= 0 && newR < nRows && newC >= 0 && newC < nCols && grid(newR)(newC) != "#") {
          guardPosition = (newR, newC)
          visited += guardPosition
        } else {
          cycle = (cycle + 1) % DIRECTIONS.length
        }

        // We've exited the grid
        if (newR < 0 || newR >= nRows || newC < 0 || newC >= nCols) {
          break
        }
      }
    }
  }


  visited.size
}
