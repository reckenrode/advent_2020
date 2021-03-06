module Advent2020.Solutions.Day5

open System
open Utilities

let name = "day5"

let mkIntpreter (fstHalf: char, sndHalf: char) n (cmd: char) =
    match cmd with
    | x when x = fstHalf -> n <<< 1
    | x when x = sndHalf -> (n <<< 1) + 1
    | _ -> failwith "invalid command"

let parseRow row = Span.fold (mkIntpreter ('F', 'B')) 0 row

let parseColumn col = Span.fold (mkIntpreter ('L', 'R')) 0 col

let seatId row col =
    row * 8 + col

let parsePass (pass: string) =
    let span = pass.AsSpan ()
    let rowPart = span.Slice (0, 7)
    let colPart = span.Slice (7, 3)
    seatId (parseRow rowPart) (parseColumn colPart)

let run (input: seq<string>, arg: string) =
    let passes = input |> Seq.map parsePass
    if arg = "max"
    then
        let maxPass = passes |> Seq.max
        printfn $"Max: {maxPass}"
    else
        let gaps =
            passes
            |> Seq.sort
            |> Seq.pairwise
            |> Seq.filter (fun (lhs, rhs) -> lhs <> rhs - 1)
            |> Seq.head
        let mySeat = (fst gaps) + 1
        printfn $"My seat: {mySeat}"
