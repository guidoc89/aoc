

@main def part2(): Long = {
  var lines = scala.io.Source.fromFile("../prod2.txt").getLines().toList
  var ans: Long = 0

  for (line <- lines) {
    val Array(left, right) = line.split(":").map(_.trim)
    val target = left.toLong
    // Splits by ANY amount of whitespace
    val nums = right.split("\\s+").map(_.toInt).toList


    if (Helpers.isLineCorrect(nums, 0, 0, target)) {
      ans += target
    }
  }



  ans
}

object Helpers {
  def isLineCorrect(nums: List[Int], idx: Int, currVal: Long, target: Long): Boolean = {
    if (idx == nums.length) {
      return currVal == target

    }

    var addRes = isLineCorrect(nums, idx+1, currVal+nums(idx), target)
    var mulRes = isLineCorrect(nums, idx+1, currVal*nums(idx), target)
    var concatRes = isLineCorrect(nums, idx+1, (currVal.toString + nums(idx).toString).toLong, target)

    return addRes || mulRes || concatRes
  }
}
