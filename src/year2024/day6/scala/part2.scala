import scala.util.control.Breaks._
import scala.collection.mutable.HashSet

@main def part2(): Int = {
  var lines = scala.io.Source.fromFile("../prod2.txt").getLines().toArray
  var grid = lines.map(_.split("").toArray)
  var ans: Int = 0

  var cycle = 0
  var nRows = grid.length
  var nCols = grid(0).length
  var originalGuardPosition = (0, 0)
  var guardPosition = (0, 0)

  breakable {
    for (i <- 0 until nRows) {
      for (j <- 0 until nCols) {
        if (grid(i)(j) == "^") {
          originalGuardPosition = (i, j)
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

    for (i <- 0 until nRows) {
      for (j <- 0 until nCols) {

          // Can process, no need for a "continue"
        if (grid(i)(j) == "." && (i,j) != originalGuardPosition) {
          var visited = HashSet[((Int, Int), Int)]()
          var cycle = 0
          guardPosition = originalGuardPosition
          grid(i)(j) = "#" 

          breakable {
            while (true) {
              var loopCondition = (guardPosition, cycle)
              if (visited.contains(loopCondition)) {
                ans += 1
                break
              }

              visited += loopCondition
              var (deltaR, deltaC) = DIRECTIONS(cycle)
              var newR = guardPosition(0) + deltaR
              var newC = guardPosition(1) + deltaC
              if (newR >= 0 && newR < nRows && newC >= 0 && newC < nCols && grid(newR)(newC) != "#") {
                guardPosition = (newR, newC)
              } else {
                cycle = (cycle + 1) % DIRECTIONS.length
              }

              if (newR < 0 || newR >= nRows || newC < 0 || newC >= nCols) {
                break
              }
            }
          }

          grid(i)(j) = "." 
        }
      }
    }


  ans
}
