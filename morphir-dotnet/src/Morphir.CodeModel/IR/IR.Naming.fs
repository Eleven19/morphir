namespace Morphir.IR

open Morphir.IR.Name

type Path = Path of Name list

type QName = QName of Path * Name

type PackageName = PackageName of Path

type ModuleName = ModuleName of Path
