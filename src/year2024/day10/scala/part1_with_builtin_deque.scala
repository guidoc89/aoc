
import scala.collection.mutable.HashSet
import scala.collection.mutable.ArrayDeque
import scala.util.control.Breaks._

@main def part1(): Int = {
  var ans: Int = 0

  var lines = scala.io.Source.fromFile("../prod1.txt").getLines().toArray
  var grid = lines.map(_.split("").map(_.toInt).toArray)

  var DIRECTIONS = List(
    (-1,0),
    (1,0),
    (0,1),
    (0,-1),
    )

  var nRows: Int = grid.length
  var nCols: Int = grid(0).length

  for (i <- 0 until nRows ) {
    for (j <- 0 until nCols ) {
      if (grid(i)(j) == 0) {
        var visited = HashSet[(Int, Int)]()
        var q = ArrayDeque((i, j))

        while (q.length > 0) {
          var (r, c) = q.removeHead()

          if (!visited.contains((r, c))) {
            visited += (r, c)
            if (grid(r)(c) == 9) {
              ans += 1
            }

            for ((deltaR, deltaC) <- DIRECTIONS) {
              var newR = r + deltaR
              var newC = c + deltaC
              if (newR >= 0 && newR < nRows && newC >= 0 && newC < nCols && grid(newR)(newC) == grid(r)(c) + 1) {
                q.append((newR, newC))
              }
            }
          }
        }
      }
    }
  }


  ans
}
