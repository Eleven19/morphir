package morphir.cli.make
import caseapp.{ExtraName => Short, HelpMessage, _}
import morphir.cli.options.OptionGroup
@HelpMessage("Compile the project into Morphir IR.")
final case class MakeOptions(
    @Group(OptionGroup.make)
    @HelpMessage("Root directory of the project where morphir.json is located. (default: \".\")")
    projectDir: String = ".",
)
