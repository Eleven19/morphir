package morphir.cli.make

import morphir.MorphirCommand
import caseapp.core.RemainingArgs

object Make extends MorphirCommand[MakeOptions] {

  override def run(options: MakeOptions, remainingArgs: RemainingArgs): Unit =
    println("Make command not yet implemented")

}
