// Copyright 2024 FINOS
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//     http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
module Morphir.IR.Name

open Morphir.SDK.Maybe
open Morphir.SDK

type Name = string list

let inline fromList (words: string list) : Name = words

let fromString (string: string) : Name =
    let wordPattern =
        Regex.fromString "([a-zA-Z][a-z]*|[0-9]+)" |> Maybe.withDefault Regex.never

    Regex.find wordPattern string
    |> List.map (fun me -> me.Match)
    |> List.map String.toLower
    |> fromList

let inline toList (name: Name) : List<string> = name

let capitalize string : string =
    match String.uncons string with
    | Just(headChar, tailString) -> String.cons (Char.toUpper headChar) tailString
    | Nothing -> string

let toTitleCase name =
    name |> toList |> List.map capitalize |> String.join ""

let toCamelCase (name: Name) =
    match name |> toList with
    | [] -> System.String.Empty
    | head :: tail -> tail |> List.map capitalize |> List.cons head |> String.join ""

let toHumanWords name : List<string> =
    let words = toList name

    let join abbrev =
        abbrev |> String.join "" |> String.toUpper

    let rec process' prefix abbrev suffix =
        match suffix with
        | [] ->
            if (List.isEmpty abbrev) then
                prefix
            else
                List.append prefix [ join abbrev ]
        | first :: rest ->
            if (String.length first = 1) then
                process' prefix (List.append abbrev [ first ]) rest
            else
                match abbrev with
                | [] -> process' (List.append prefix [ first ]) [] rest
                | _ -> process' (List.append prefix [ join abbrev; first ]) [] rest

    process' [] [] words

let toSnakeCase name = name |> toHumanWords |> String.join "_"
