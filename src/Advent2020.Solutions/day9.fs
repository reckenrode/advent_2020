module Advent2020.Solutions.Day9

open FSharpx
open FSharpx.Collections

module XmasCracker =
    let rec private calculateSums window = function
    | [] -> LazyList.empty
    | x::xs ->
        let xSums = xs |> Seq.truncate (window - 1) |> Seq.map ((+) x) |> LazyList.ofSeq
        LazyList.consDelayed xSums (fun () -> (xs |> calculateSums window))

    let findNonSumming window data =
        let rest = data |> Seq.skip window
        let sums = data |> calculateSums window
        let rec findNonSumming' window data sums =
            match data |> Seq.unCons with
            | Some (x, xs) ->
                let windowSums =
                    sums
                    |> LazyList.take window
                    |> LazyList.tryFind (LazyList.tryFind ((=) x) >> Option.isSome)
                if windowSums |> Option.isNone
                then Some x
                else findNonSumming' window xs (sums |> LazyList.tail)
            | _ -> None
        findNonSumming' window rest sums

let name = "day9"

let run (input: seq<string>, arg: string) =
    match input |> Seq.map Utilities.tryParse |> List.ofSeq |> Option.sequence with
    | None -> printfn "error reading list of numbers"
    | Some (nums: list<int64>) ->
        match nums |> XmasCracker.findNonSumming 25 with
        | None -> printfn "All the numbers are good. No bad number found."
        | Some badNum ->
            printfn $"The bad number is {badNum}"

