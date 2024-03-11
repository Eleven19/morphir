namespace Morphir.IR


module Path =
    open Morphir.IR.Name
    open Morphir.SDK.List
    open Morphir.SDK

    /// <summary>
    /// Type that represents a path as a list of names.
    /// </summary>
    [<Struct>]
    type Path =
        | Path of Names: Name list

        static member inline FromList(names: Name list) : Path = Path names

        static member FromList(input: string list list) : Path =
            input |> List.map Name.fromList |> Path.FromList

    let inline fromList (names: List<Name>) : Path = Path names

    let inline toList (Path names) : List<Name> = names

    let toString nameToString sep path =
        path |> toList |> List.map nameToString |> String.join sep

    let fromString string =
        let separatorRegex = Regex.fromString "[^\\w\\s]+" |> Maybe.withDefault Regex.never in

        Regex.split separatorRegex string |> List.map Name.fromString |> fromList

    let rec isPrefixOf (Path path) (Path prefix) =
        let rec loop path prefix =
            match (path, prefix) with
            // empty path is a prefix of any other path
            | ([], _) -> true
            // empty path has no prefixes except the empty prefix captured above
            | (_, []) -> false
            // for any other case compare the head and recurse
            | (pathHead :: pathTail, prefixHead :: prefixTail) ->
                if prefixHead = pathHead then
                    loop prefixTail pathTail
                else
                    false

        loop path prefix

    let inline isPrefixFor prefix path = isPrefixOf path prefix

type PathBuilder() =
    [<CustomOperation("name")>]
    member inline _.Name((), parts: string list) = Name.fromList parts

    [<CustomOperation("name")>]
    member inline _.Name((), nameStr: string) = Name.fromString nameStr

    member inline _.Yield(()) = ()
