module Advent2020.Solutions.Day10

open FSharpx
open MathNet.Numerics.LinearAlgebra

#nowarn "25"
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
        |> Seq.map ((flip >> uncurry) (-))
        |> Seq.countBy id
        |> Map.ofSeq

    let private buildGraph size lst =
        let findAdjacents cur = List.takeWhile (fun x -> x - cur <= 3)
        let rec loop acc = function
            | [] -> acc
            | x::xs -> xs |> loop ((x, (findAdjacents x xs))::acc)
        lst |> loop [] |> Map.ofList

    let countWalks startNode endNode adjacencyList =
        let size = endNode + 1
        let mutable graph = Matrix<float>.Build.Dense (size, size, 0.0)
        adjacencyList |> Map.toList |> List.iter (fun (x, adjacents) ->
            adjacents |> List.iter (fun a ->
                graph.[x, a] <- 1.0))
        let rec countWalks' longestWalk acc (walked: Matrix<float>) =
            if longestWalk = 0
            then acc |> int64
            else graph * walked |> countWalks' (longestWalk - 1) (acc + walked.[startNode, endNode])
        graph |> countWalks' (adjacencyList |> Map.count) 0.0

    let countArrangements socketJoltage adapters =
        let (x::xs) = socketJoltage::adapters |> List.sort |> List.rev
        let deviceJoltage = x + 3
        let sorted = deviceJoltage::x::xs |> List.rev
        let graph = sorted |> buildGraph (deviceJoltage + 1)
        graph |> countWalks socketJoltage deviceJoltage

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

            let walks = chain |> AdapterAnalyzer.countArrangements 0
            printfn $"The number of different ways to arrange the adapters is: {walks}"
