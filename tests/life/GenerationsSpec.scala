package life

import org.scalatest.*
import org.scalatest.matchers.should.*
import org.scalatest.wordspec.*

class GenerationsSpec extends AnyWordSpec with Matchers:

  "Generations" should {

    "be an iterator" in {
      val world       = World.withCells(Patterns.block)
      val generations = Generations(world)
      generations should be(a[Iterator[World]])
    }

    "iterate to the first world" in {
      val world       = World.withCells(Patterns.toad)
      val generations = Generations(world)
      generations.hasNext should be(true)
      generations.next() should be(world)
    }

    "iterate to the next generation" in {
      val world       = World.withCells(Patterns.toad)
      val generations = Generations(world)
      generations.hasNext should be(true)
      generations.next() should be(world)
      generations.hasNext should be(true)
      generations.next() should be(world.nextGeneration)
    }

    "not have a next available after iterating back to a former world" in {
      val world       = World.withCells(Patterns.block)
      val generations = Generations(world)
      generations.hasNext should be(true)
      generations.next() should be(world)
      generations.hasNext should be(true)
      generations.next() should be(world.nextGeneration)
      generations.hasNext should be(false)
    }

    "stop when the World is dead" in {
      val world       = World.withCells(Set(Cell(0, 0)))
      val generations = Generations(world)
      generations.hasNext should be(true)
      generations.next() should be(world)
      generations.hasNext should be(true)
      generations.next() should be(world.nextGeneration)
      generations.hasNext should be(false)

    }

  }
