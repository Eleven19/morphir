package morphir
import caseapp._
import caseapp.core.app.CommandsEntryPoint
import morphir.cli.*

object Main extends CommandsEntryPoint {
  val progName = "morphir"

  override val description =
    """|The morphir cli is a tool for working with morphir projects and models.
       |It enables you to create, build, and test morphir models. 
       |""".stripMargin

  val commands = Seq(
    make.Make,
    restore.Restore
  )
}
