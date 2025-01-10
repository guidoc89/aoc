
@main def part1(): Unit  =
  val lines = scala.io.Source.fromFile("../prod1.txt").getLines().toList


  var L = scala.collection.mutable.ArrayBuffer[Int]()
  var R = scala.collection.mutable.ArrayBuffer[Int]()
  var ans = 0

  for (line <- lines) {
    val nums = line.split(" +").map(_.toInt).toList
    L += nums(0)
    R += nums(1)
  }

  L.sortInPlace()
  R.sortInPlace()

  for ((l,r) <- L.zip(R)) {
    ans += (l-r).abs
  }

    println(ans)

