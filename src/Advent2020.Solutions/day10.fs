module Advent2020.Solutions.Day10

open FSharpx

module AdapterAnalyzer =
    let createChain socketJoltage adapters =
        match adapters |> List.sort with
        | (x::_) as result when (x - socketJoltage) <= 3 -> Some result
        | _ -> None

    let findDifferences socketJoltage = function
    | [] -> Map.empty
    | adapters ->
        let deviceJoltage = (adapters |> List.last) + 3
        let chain = Seq.concat [[socketJoltage]; adapters; [deviceJoltage]]
        chain
        |> Seq.pairwise
        |> Seq.map (fun (lhs, rhs) -> rhs - lhs)
        |> Seq.countBy id
        |> Map.ofSeq

let name = "day10"

let run (input: seq<string>, arg: string) =
    match input |> Seq.map Utilities.tryParse |> List.ofSeq |> Option.sequence with
    | None -> printfn "Error: failed to read input file"
    | Some adapters ->
        match adapters |> AdapterAnalyzer.createChain 0 with
        | None -> printfn "Error: adapters were not capable of making a chain (bad input?)"
        | Some chain ->
            let statistics = chain |> AdapterAnalyzer.findDifferences 0
            let oneCount = statistics |> Map.tryFind 1 |> Option.getOrElse 0
            let threeCount = statistics |> Map.tryFind 3 |> Option.getOrElse 0
            printfn $"The distribution is: {statistics}"
            printfn $"The product of the 1- and 3-counts is: {oneCount * threeCount}"
