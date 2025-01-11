
@main def part1(): Int = {
  val lines = scala.io.Source.fromFile("../prod1.txt").getLines().toList
  var ans: Int = 0

  for (line <- lines) {
    val l_length: Int = line.length
    for (i <- 0 until line.length) {
      if (line.slice(i, l_length).startsWith("mul(")) {
        var end: Int = i
        while (line.charAt(end) != ')') {
          end += 1
        }

        val new_slice = line.slice(i, end+1).replace("mul(", "").replace(")", "")
        var splits = new_slice.split(",")
        // NOTE: could also do it with scala.util.Try -> `Try(splits(0).toInt).isSuccess == Boolean`
        if (splits.length == 2 && splits(0).toIntOption.isDefined && splits(1).toIntOption.isDefined) {
              val n1 = splits(0).toInt
              val n2 = splits(1).toInt
              ans += (n1 * n2)
        }

      }
    }
  }

  ans
}
