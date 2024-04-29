package morphir.cli.restore
import caseapp.*
import morphir.MorphirCommand

object Restore extends MorphirCommand[RestoreOptions] {
  def run(options: RestoreOptions, remainingArgs: RemainingArgs): Unit = {
    println("Options: " + options)
    println("Restore command not yet implemented")
  }
}
