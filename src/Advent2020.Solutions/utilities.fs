module Advent2020.Solutions.Utilities

open System.IO

let liftResult f x =
    try
        f x |> Ok
    with
    |   ex -> Error ex

let readFile: string -> Result<seq<string>, exn> = liftResult File.ReadLines
