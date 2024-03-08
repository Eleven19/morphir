module Morphir.Utils.String

open System

let (|IsNullOrEmpty|) (s: string) =
    if String.IsNullOrEmpty(s) then None else Some s

let (|IsNullOrWhiteSpace|) (s: string) =
    if String.IsNullOrWhiteSpace(s) then None else Some s

let (|NonEmpty|) (s: string option) =
    match s with
    | Some(IsNullOrWhiteSpace s) -> None
    | s -> s
