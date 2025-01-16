
@main def part1(): Int  = {
  val lines = scala.io.Source.fromFile("../test1.txt").getLines().toList
  var ans: Int = 0

  for (line <- lines) {
    val nums = line.split(" +").map(_.toInt).toList

    if (Helpers.isSafe(nums)) {
      println("Is safe!")
      ans += 1
    }
  }

  ans
}


object Helpers {
  def isSafe(nums: List[Int]): Boolean = {
    var allPositive = true
    var allNegative = true
    var maxThree = true
    var minOne = true

    for (i <- 1 until nums.length) {
      val diff = nums(i-1) - nums(i)

      if (diff <= 0) {
        allPositive = false
      }

      if (diff >= 0) {
        allNegative = false
      }

      val abs_diff = diff.max(-diff)

      if (abs_diff < 1) {
        minOne = false

      }

      if (abs_diff > 3) {
        maxThree = false
      }
    }

    (allNegative || allPositive) && maxThree && minOne
  }
}
