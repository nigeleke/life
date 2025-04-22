package life

import com.typesafe.config.*

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class PatternsSpec extends AnyWordSpec with Matchers:

  "Patterns" should {
    "be provided" when {
      "beacon" in {
        Patterns.beacon should contain theSameElementsAs (Cells
          .fromText("""
          |* * . .
          |* . . *
          |. . * *
          |""".stripMargin)
          .getOrElse(???))
      }

      "beehive" in {
        Patterns.beehive should contain theSameElementsAs (Cells
          .fromText("""
          |. * * .
          |* . . *
          |. * * .
          |""".stripMargin)
          .getOrElse(???))
      }

      "blinker" in {
        Patterns.blinker should contain theSameElementsAs (Cells.fromText("* * *").getOrElse(???))
      }

      "block" in {
        Patterns.block should contain theSameElementsAs (Cells
          .fromText("""
          |* *
          |* *
          |""".stripMargin)
          .getOrElse(???))
      }

      "boat" in {
        Patterns.boat should contain theSameElementsAs (Cells
          .fromText("""
          |* * .
          |* . *
          |. * .
          |""".stripMargin)
          .getOrElse(???))
      }

      "glider" in {
        Patterns.glider should contain theSameElementsAs (Cells
          .fromText("""
          |* * *
          |. . *
          |. * .
          |""".stripMargin)
          .getOrElse(???))
      }

      "gospar glider gun" in {
        Patterns.gliderGunGosper should contain theSameElementsAs (Cells
          .fromText("""
          |. . . . . . . . . . . . . . . . . . . . . . . . * . . . . . . . . . . .
          |. . . . . . . . . . . . . . . . . . . . . . * . * . . . . . . . . . . .
          |. . . . . . . . . . . . * * . . . . . . * * . . . . . . . . . . . . * *
          |. . . . . . . . . . . * . . . * . . . . * * . . . . . . . . . . . . * *
          |* * . . . . . . . . * . . . . . * . . . * * . . . . . . . . . . . . . .
          |* * . . . . . . . . * . . . * . * * . . . . * . * . . . . . . . . . . .
          |. . . . . . . . . . * . . . . . * . . . . . . . * . . . . . . . . . . .
          |. . . . . . . . . . . * . . . * . . . . . . . . . . . . . . . . . . . .
          |. . . . . . . . . . . . * * . . . . . . . . . . . . . . . . . . . . . .
          |""".stripMargin)
          .getOrElse(???))
      }

      "loaf" in {
        Patterns.loaf should contain theSameElementsAs (Cells
          .fromText("""
          |. * * .
          |* . . *
          |. * . *
          |. . * .
          |""".stripMargin)
          .getOrElse(???))
      }

      "penta decathlon" in {
        Patterns.pentaDecathlon should contain theSameElementsAs (Cells
          .fromText("""
          |* * *
          |* . *
          |* * *
          |* * *
          |* * *
          |* . *
          |* * *
          |""".stripMargin)
          .getOrElse(???))
      }

      "pulsar" in {
        Patterns.pulsar should contain theSameElementsAs (Cells
          .fromText("""
          |. . * * * . . . * * * . .
          |. . . . . . . . . . . . .
          |* . . . . * . * . . . . *
          |* . . . . * . * . . . . *
          |* . . . . * . * . . . . *
          |. . * * * . . . * * * . .
          |. . . . . . . . . . . . .
          |. . * * * . . . * * * . .
          |* . . . . * . * . . . . *
          |* . . . . * . * . . . . *
          |* . . . . * . * . . . . *
          |. . . . . . . . . . . . .
          |. . * * * . . . * * * . .
          |""".stripMargin)
          .getOrElse(???))
      }

      "random" in {
        val config = ConfigFactory.load().getConfig("life")
        val board  = Bounds.from(config.getConfig("defaults.boardBounds"))
        val cells  = Patterns.random(board)
        cells should be(a[Cells])
        cells should not be (empty) // Possible false failure
      }

      "spaceship lightweight" in {
        Patterns.spaceshipLightweight should contain theSameElementsAs (Cells
          .fromText("""
          |* . . * .
          |. . . . *
          |* . . . *
          |. * * * *
          |""".stripMargin)
          .getOrElse(???))
      }

      "toad" in {
        Patterns.toad should contain theSameElementsAs (Cells
          .fromText("""
          |. * * *
          |* * * .
          |""".stripMargin)
          .getOrElse(???))
      }

      "tub" in {
        Patterns.tub should contain theSameElementsAs (Cells
          .fromText("""
          |. * .
          |* . *
          |. * .
          |""".stripMargin)
          .getOrElse(???))
      }

    }
  }
