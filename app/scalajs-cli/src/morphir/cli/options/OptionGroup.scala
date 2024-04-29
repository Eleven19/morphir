package morphir.cli.options

object OptionGroup {
  val help = "Help"
  val verbosity = "Verbosity"
  val setup = "Setup"
  val restore = "Restore"
  val make = "Make"

  val order: Seq[String] = Seq(
    help,
    verbosity,
    restore,
    setup,
    make
  )

}
