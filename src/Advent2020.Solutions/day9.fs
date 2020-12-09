module Advent2020.Solutions.Day9

open Advent2020.Solutions.Utilities
open FSharpx
open FSharpx.Collections
open FsRandom

module XmasCracker =
    let rec private calculateSums window = function
    | [] -> LazyList.empty
    | x::xs ->
        let xSums = xs |> Seq.truncate (window - 1) |> Seq.map ((+) x) |> LazyList.ofSeq
        LazyList.consDelayed xSums (fun () -> (xs |> calculateSums window))

    let findNonSumming window data =
        let rest = data |> List.skip window
        let sums = data |> calculateSums window
        let rec findNonSumming' window sums = function
            | x::xs ->
                let windowSums =
                    sums
                    |> LazyList.take window
                    |> LazyList.tryFind (LazyList.tryFind ((=) x) >> Option.isSome)
                if windowSums |> Option.isNone
                then Some x
                else xs |> findNonSumming' window (sums |> LazyList.tail)
            | [] -> None
        rest |> findNonSumming' window sums

    let findWeakSequence badNumber (data: list<int64>) =
        let arr = data |> Array.ofList
        let rng = Utility.createRandomState ()
        let distance target (ps: list<int>) (data: array<int64>) =
            let lower = min ps.[0] ps.[1]
            let upper = max ps.[0] ps.[1]
            let span = System.ReadOnlySpan data
            let slice = span.Slice (lower, upper - lower)
            abs (target - Span.fold (+) 0L slice)
        let predicate = List.forall (fun x -> data.[x] <> badNumber)
        let result = rng |> Random.get (Day1.findSolutionGeneric distance predicate badNumber arr 2)
        let lower = List.min result
        let upper = List.max result
        data |> List.skip lower |> List.take (upper - lower)

let name = "day9"

let run (input: seq<string>, arg: string) =
    match input |> Seq.map Utilities.tryParse |> List.ofSeq |> Option.sequence with
    | None -> printfn "error reading list of numbers"
    | Some (nums: list<int64>) ->
        match nums |> XmasCracker.findNonSumming 25 with
        | None -> printfn "All the numbers are good. No bad number found."
        | Some badNum ->
            printfn $"The bad number is {badNum}"
            let weakSequence = nums |> XmasCracker.findWeakSequence badNum
            let smallest = List.min weakSequence
            let largest = List.max weakSequence
            printfn $"The sum of the smallest and larger numbers is {smallest + largest}"

