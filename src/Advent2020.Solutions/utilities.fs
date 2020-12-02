module Advent2020.Solutions.Utilities

open System.IO

let liftOption xs =
    Seq.foldBack (fun x xs ->
        match xs with
        | Some xs -> x |> Option.bind (fun o -> o::xs |> Some)
        | None -> None) xs (Some [])

let liftResult f x =
    try
        f x |> Ok
    with
    |   ex -> Error ex

let readFile: string -> Result<seq<string>, exn> = liftResult File.ReadLines
