package morphir.cli.options

object OptionGroup {
  val help = "Help"
  val verbosity = "Verbosity"
  val setup = "Setup"
  val restore = "Restore"

  val order: Seq[String] = Seq(
    help,
    verbosity,
    restore,
    setup
  )

}
