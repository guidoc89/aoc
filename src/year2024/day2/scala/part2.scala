
@main def part2(): Int  = {
  val lines = scala.io.Source.fromFile("../prod2.txt").getLines().toList
  var ans: Int = 0

  for (line <- lines) {
    var safe: Boolean = false
    val nums = line.split(" +").map(_.toInt).toList
    if (Helpers.is_safe(nums)) {
      safe = true
    } else {
      for (i <- 0 until nums.length) {
        val new_nums = nums.slice(0,i) ++ nums.slice(i+1,nums.length)
        if (Helpers.is_safe(new_nums)) {
          safe = true
        }
      }
    }

    if (safe) {
      ans += 1
    }
  }

  ans
}


object Helpers {
  def is_safe(nums: List[Int]): Boolean = {
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
