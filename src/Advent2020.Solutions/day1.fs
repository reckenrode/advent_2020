module Advent2020.Solutions.Day1

open FsRandom

let name = "day1"

let targetNumber = 2020

let distance target (x, y) (data: array<int>) =
    target - (data.[x] + data.[y]) |> System.Math.Abs

let enumerateNeighbors (pos&(x, y)) =
    Seq.allPairs [x-1; x; x+1] [y-1; y; y+1]
    |> Seq.filter (fun p -> p <> pos)

let rec climb target pos currentDistance data =
    random {
        if currentDistance = 0
        then return Some pos
        else
            let maxPos = data |> Array.length
            let possibilities =
                enumerateNeighbors pos
                |> Seq.filter (fun (x, y) -> x >= 0 && x < maxPos && y >= 0 && y < maxPos)
                |> Seq.map (fun pos -> (pos, distance target pos data))
                |> Seq.filter (fun (_, d) -> d < currentDistance)
                |> Array.ofSeq
            if possibilities |> Array.length = 0
            then return None
            else
                let! candidate = possibilities |> Array.sampleOne
                let newPos, newDistance = candidate
                return! data |> climb target newPos newDistance
    }

let rec findSolution target numbers =
    random {
        let maxPos = (numbers |> Array.length) - 1
        let! startX = Statistics.uniformDiscrete (0, maxPos)
        let! startY = Statistics.uniformDiscrete (0, maxPos)
        let start = (startX, startY)
        match! climb target start (distance target start numbers) numbers with
        | Some result -> return result
        | None -> return! findSolution target numbers
    }

let run (input: seq<string>, part: string) =
    let numbers: option<list<int>> = input |> Seq.map Utilities.tryParse |> Utilities.liftOption
    match numbers with
    | None -> printfn "Error: input file not in expected format (a list of integers)."
    | Some numbers ->
        let numbers = numbers |> List.sort |> Array.ofList
        let rng = Utility.createRandomState ()
        let result = rng |> Random.get (findSolution targetNumber numbers)
        let n1 = numbers.[fst result]
        let n2 = numbers.[snd result]
        printfn $"{n1} × {n2} = {n1 * n2}"
