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

let inline tryParse (str: string) : option<'a>
        when ^a: (static member TryParse: string * ^a byref -> bool) =
    let mutable result = Unchecked.defaultof<'a>
    let didParse = (^a: (static member TryParse: string * ^a byref -> bool) (str, &result))
    if didParse then Some result else None
