
@main def part2(): Unit  =
  val lines = scala.io.Source.fromFile("../prod2.txt").getLines().toList


  var L = scala.collection.mutable.ArrayBuffer[Int]()
  var counts = scala.collection.mutable.Map[Int, Int]()
  var ans = 0

  for (line <- lines) {
    val nums = line.split(" +").map(_.toInt).toList
    L += nums(0)

    counts(nums(1)) = counts.getOrElse(nums(1), 0) + 1
  }

  for (l <- L) {
    ans += (l * counts.getOrElse(l,0))
  }

  println(ans)

