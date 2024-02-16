module Morphir.Runtime.RuntimeExtension

open System
open System.Runtime.InteropServices
open Extism

let Eval () : int32 =
    let input = Pdk.GetInputString()
    let greeting = $"Hello, {input}!"
    Pdk.SetOutput greeting
    0
