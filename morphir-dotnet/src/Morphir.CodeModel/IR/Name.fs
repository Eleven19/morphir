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

open System
open System.Text.RegularExpressions
open Morphir.Extensions
open Morphir.SDK
open Morphir.SDK.Maybe

/// Type that represents a name that is made up of words.
type Name = string list


/// Translate a string into a name by splitting it into words. The algorithm is designed
/// to work with most well-known naming conventions or mix of them. The general rule is that
/// consecutive letters and numbers are treated as words, upper-case letters and non-alphanumeric
/// characters start a new word.
let fromString (input: string) : Name =
    let wordPattern = new Regex("([a-zA-Z][a-z]*|[0-9]+)")

    wordPattern.Matches(input)
    |> Seq.cast<Match>
    |> Seq.map (fun m -> m.Value.ToLower())
    |> Seq.toList

/// Turns a name into a list of human-readable strings. The only difference
/// compared to `toList` is how it handles abbreviations. They will
/// be turned into a single upper-case word instead of separate single-character
/// words.
let toHumanWords (name: Name) : string list =
    let join (abbrev: string list) : string =
        abbrev |> List.reduce (+) |> String.ToUpper

    let rec loop (prefix: string list) (abbrev: string list) (suffix: string list) : string list =
        match suffix with
        | [] ->
            if List.isEmpty abbrev then
                prefix
            else
                prefix @ [ join abbrev ]
        | first :: rest ->
            if String.length first = 1 then
                loop prefix (abbrev @ [ first ]) rest
            else
                match abbrev with
                | [] -> loop (prefix @ [ first ]) [] rest
                | _ -> loop (prefix @ [ join abbrev; first ]) [] rest

    match name with
    | [ word ] -> if String.length word = 1 then name else loop [] [] name
    | _ -> loop [] [] name

/// Turns a name into a title-case string.
let toTitleCase (name: Name) : string =
    name
    |> List.map (fun word -> word.[0..0].ToUpper() + word.[1..])
    |> String.concat ""

/// Turns a name into a camel-case string.
let toCamelCase (name: Name) : string =
    match name with
    | [] -> ""
    | head :: tail ->
        tail
        |> List.map (fun word -> word.[0..0].ToUpper() + word.[1..])
        |> List.reduce (+) head

/// Turns a name into a snake-case string.
let toSnakeCase (name: Name) : string =
    name |> toHumanWords |> String.concat "_"



/// Turns a name into a list of human-readable strings with the first word capitalized. The only difference
/// compared to `toList` is how it handles abbreviations. They will
/// be turned into a single upper-case word instead of separate single-character
/// words.
let toHumanWordsTitle (name: Name) : string list =
    match toHumanWords name with
    | firstWord :: rest -> (firstWord.[0..0].ToUpper() + firstWord.[1..]) :: rest
    | [] -> []

/// Convert a list of strings into a name.
let fromList (words: string list) : Name = words

/// Convert a name to a list of strings.
let toList (name: Name) : string list = name
