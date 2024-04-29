package morphir
import caseapp._
import caseapp.core.help.HelpFormat
import caseapp.core.Scala3Helpers._
import morphir.cli.options.OptionGroup

abstract class MorphirCommand[T](implicit parser: Parser[T], help: Help[T])
    extends Command[T]()(parser, help) {

  override def helpFormat: HelpFormat =
    HelpFormat.default().withSortedGroups(Some(OptionGroup.order))

  override def hasFullHelp = true
}
