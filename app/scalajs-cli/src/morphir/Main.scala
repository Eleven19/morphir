package morphir
import mainargs.{main, arg, ParserForMethods, ParserForClass, Flag}

object Main {
  @main
  case class Config(
      @arg(short = 'f', doc = "String to print repeatedly")
      foo: String,
      @arg(doc = "How many times to print string")
      myNum: Int = 2,
      @arg(doc = "Example flag")
      bool: Flag
  )

  given ParserForClass[Config] = ParserForClass[Config]

  @main
  def bar(
      config: Config,
      @arg(name = "extra-message")
      extraMessage: String
  ) = {
    println(
      config.foo * config.myNum + " " + config.bool.value + " " + extraMessage
    )
  }
  @main
  def qux(config: Config, n: Int) = {
    println((config.foo * config.myNum + " " + config.bool.value + "\n") * n)
  }

  def main(args: Array[String]): Unit =
    ParserForMethods(this).runOrExit(args)
}
