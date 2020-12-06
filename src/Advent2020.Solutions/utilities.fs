module Advent2020.Solutions.Utilities

open FSharpx.Collections

open System.IO

let cycle xs =
    let originalXs = xs
    let rec cycle' xs =
        seq {
            match xs |> Seq.unCons with
            | Some (head, tail) ->
                yield head
                if tail |> Seq.isEmpty
                then yield! cycle' originalXs
                else yield! cycle' tail
            | None -> failwith "empty sequences are not supported"
        }
    cycle' xs

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

let rec pairs = function
| [] -> []
| [x] -> [x]
| x::[y] -> List.allPairs x y |> List.map (fun (x, y) -> [x; y])
| x::xs -> pairs xs  |> List.allPairs x |> List.map List.Cons

let read: string -> Result<string, exn> = liftResult File.ReadAllText

let readFile: string -> Result<seq<string>, exn> = liftResult File.ReadLines

let inline tryParse (str: string) : option<'a>
        when ^a: (static member TryParse: string * ^a byref -> bool) =
    let mutable result = Unchecked.defaultof<'a>
    let didParse = (^a: (static member TryParse: string * ^a byref -> bool) (str, &result))
    if didParse then Some result else None

let rec trySkip n xs =
    if xs |> Seq.isEmpty
    then Seq.empty
    else
        let t = xs |> Seq.tail
        if n = 1
        then t
        else trySkip (n - 1) t

let rec allUnique = function
| [] -> true
| x::xs-> xs |> List.forall (fun y -> x <> y) && allUnique xs

module Result =
    let defaultWith f = function
    | Ok x -> x
    | Error ex -> (f ex)
