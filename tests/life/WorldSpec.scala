package life

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class WorldSpec extends AnyWordSpec with Matchers:

  "A World" should {

    "allow an initial set of cells to be provided" in {
      val cells = Set(Cell(0, 0))
      val world = World.withCells(cells)
      world.liveCells should contain theSameElementsAs (cells)
    }

    "allow limits to be defined" when {
      "all enclosing" in {
        val bounds = Bounds(0 until 10, 0 until 10)
        for cells <- Cells.fromText("""
          |* * *
          |. . *
          |. * .
          |""".stripMargin)
        yield
          val world = World.withCells(cells).withBounds(bounds)
          world.maybeBounds should be(Some(bounds))
      }

      "restrictive" in {
        val bounds = Bounds(0 until 2, 0 until 2)
        for
          initialCells <- Cells.fromText("""
            |* * *
            |. . *
            |. * .
            |""".stripMargin)
          expectedCells <- Cells.fromText("""
            |* *
            |. .
            |""".stripMargin)
        yield
          val world = World.withCells(initialCells).withBounds(bounds)
          world.liveCells should be(expectedCells)
      }

      "previously defined" in {
        val initialBounds = Bounds(0 until 10, 0 until 10)
        val newBounds     = Bounds(0 until 5, 0 until 5)
        val world         = World.withCells(Set.empty).withBounds(initialBounds).withBounds(newBounds)
        world.maybeBounds should be(Some(newBounds))
      }
    }

    "allow limits to be removed" in {
      val initialBounds = Bounds(0 until 10, 0 until 10)
      val newBounds     = Bounds(0 until 5, 0 until 5)
      val world         = World.withCells(Set.empty).withBounds(initialBounds).unbounded
      world.maybeBounds should be(None)
    }

    "allow additional cells to be added" when {
      "unbounded world" in {
        for
          initialCells <- Cells.fromText("""
            |* * *
            |. . *
            |. * .
            |""".stripMargin)
          additionalCells <- Cells.fromText("""
            |* * *
            |* . .
            |. * .
            |""".stripMargin)
          expectedCells <- Cells.fromText("""
              |* * * . . .
              |. . * * * *
              |. * . * . .
              |. . . . * .
              |""".stripMargin)
        yield
          val world = World.withCells(initialCells).withCells(additionalCells, Position(1, 3))
          world.liveCells should be(expectedCells)
      }

      "bounded world" in {
        val bounds = Bounds(0 until 3, 0 until 5)
        for
          initialCells <- Cells.fromText("""
            |* * *
            |. . *
            |. * .
            |""".stripMargin)
          additionalCells <- Cells.fromText("""
            |* * *
            |* . .
            |. * .
            |""".stripMargin)
          expectedCells <- Cells.fromText("""
            |* * * . .
            |. . * * *
            |. * . * .
            |""".stripMargin)
        yield
          val world = World.withCells(initialCells).withBounds(bounds).withCells(additionalCells, Position(1, 3))
          world.liveCells should contain theSameElementsAs (expectedCells)
      }
    }

    "allow a viewport to be defined" in {
      val viewport = Bounds(0 to 10, 0 to 10)
      val world    = World.withCells(Set.empty).withViewport(viewport)
      world.maybeViewport should be(Some(viewport))
    }

    "allow a viewport to be removed" in {
      val viewport = Bounds(0 to 10, 0 to 10)
      val world    = World.withCells(Set.empty).withViewport(viewport).withoutViewport
      world.maybeViewport should be(None)
    }

    "be pretty printable" when {
      "no viewport defined" in {
        for cells <- Cells.fromText("* . * .")
        yield
          val prettified = World.withCells(cells).prettified
          prettified should be("*   * ")
      }

      "viewport defined" in {
        for initialCells <- Cells.fromText("""
            |. . . . . .|
            |* . . . . .|
            |* * . . . .|
            |. . * . . .|
            |* . * . . .|
            |* * * . . .|
            |""".stripMargin)
        yield
          val actual = World.withCells(initialCells).withViewport(Bounds(2 until 4, 1 until 3)).prettified
          val expected =
            """*...
              |..*.""".stripMargin.replaceAll("\\.", " ").replaceAll("\r", "")
          actual should be(expected)
      }
    }
  }
