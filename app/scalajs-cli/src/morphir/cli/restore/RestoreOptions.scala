package morphir.cli.restore
import morphir.cli.options.OptionGroup

import caseapp.{ExtraName => Short, HelpMessage => Help, _}

// format: off
@Help(
  "Restore project dependencies found in morphir.json and elm.json.\n" +
  "\n" +
  "Examples:\n" +
  "$ morphir restore\n"
)
final case class RestoreOptions(
    @Group(OptionGroup.restore)
    @Help("Skip restoring Elm dependencies")
    noElm: Boolean = false
)


