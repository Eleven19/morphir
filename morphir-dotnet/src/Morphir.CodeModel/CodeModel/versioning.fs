module Morphir.CodeModel.Versioning

type Version =
    | SemanticVersion of Major: int * Minor: int * Patch: int * PreRelease: string option * Build: string option
    | MajorVersion of Major: int
    | MajorMinorVersion of Major: int * Minor: int
    | ThreePartVersion of Major: int * Minor: int * Patch: int
    
type VersioningError =
    | InvalidVersion of string
    | InvalidMajorVersion of string
    | InvalidMinorVersion of string
    | InvalidPatch of string
    | InvalidPreRelease of string
    | InvalidBuild of string

type PartialVersion =
    { Major: int option
      Minor: int option
      Patch: int option
      PreRelease: string option
      Build: string option }
    
    member x.Tupled() = (x.Major, x.Minor, x.Patch, x.PreRelease, x.Build)

let partialVersionToVersion (partialVersion:PartialVersion): Result<Version, VersioningError> =
    match partialVersion.Tupled() with
    | (Some major, Some minor, Some patch, None, None) -> Ok (ThreePartVersion(major, minor, patch))
    | (Some major, Some minor, Some patch, preRelease, build) -> Ok (SemanticVersion(major, minor, patch, preRelease, build))
    | (Some major, None, None, _, _) -> Ok (MajorVersion(major))
    | (Some major, Some minor, None, _, _) -> Ok (MajorMinorVersion(major, minor))