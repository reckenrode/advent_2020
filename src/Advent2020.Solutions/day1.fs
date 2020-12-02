module Advent2020.Solutions.Day1

open FsRandom

let name = "day1"

let targetNumber = 2020

let distance target (x, y, z) (data: array<int>) =
    let zValue = data |> Array.tryItem z |> Option.orElse (Some 0) |> Option.get
    target - (data.[x] + data.[y] + zValue) |> System.Math.Abs

let enumerateNeighbors (pos&(x, y, z)) =
    Seq.allPairs [x-1; x; x+1] [y-1; y; y+1]
    |> Seq.allPairs [z-1; z; z+1]
    |> Seq.map (fun (x, (y, z)) -> (x, y, z))
    |> Seq.filter (fun p -> p <> pos)

let isInRange bot top x =
    x >= bot && x < top

let rec climb target pos currentDistance holdZ data =
    random {
        if currentDistance = 0
        then return Some pos
        else
            let maxPos = data |> Array.length
            let isInRange = isInRange 0 maxPos
            let possibilities =
                enumerateNeighbors pos
                |> Seq.filter (fun (x, y, z) ->
                    isInRange x && isInRange y && if holdZ then z = -1 else isInRange z)
                |> Seq.map (fun pos -> (pos, distance target pos data))
                |> Seq.filter (fun (_, d) -> d < currentDistance)
                |> Array.ofSeq
            if possibilities |> Array.length = 0
            then return None
            else
                let! candidate = possibilities |> Array.sampleOne
                let newPos, newDistance = candidate
                return! data |> climb target newPos newDistance holdZ
    }

let rec findSolution target numbers holdZ =
    random {
        let maxPos = (numbers |> Array.length) - 1
        let! startX = Statistics.uniformDiscrete (0, maxPos)
        let! startY = Statistics.uniformDiscrete (0, maxPos)
        let! startZ = if holdZ then random { return -1 } else Statistics.uniformDiscrete (0, maxPos)
        let start = (startX, startY, startZ)
        match! numbers |> climb target start (distance target start numbers) holdZ with
        | Some result -> return result
        | None -> return! findSolution target numbers holdZ
    }

let run (input: seq<string>, part: string) =
    let holdZ = part <> "part2"
    let numbers: option<list<int>> = input |> Seq.map Utilities.tryParse |> Utilities.liftOption
    match numbers with
    | None -> printfn "Error: input file not in expected format (a list of integers)."
    | Some numbers ->
        let numbers = numbers |> List.sort |> Array.ofList
        let rng = Utility.createRandomState ()
        let (idx1, idx2, idx3) = rng |> Random.get (findSolution targetNumber numbers holdZ)
        let n1 = numbers.[idx1]
        let n2 = numbers.[idx2]
        if idx3 = -1
        then printfn $"{n1} × {n2} = {n1 * n2}"
        else
            let n3 = numbers.[idx3]
            printfn $"{n1} × {n2} × {n3} = {n1 * n2 * n3}"
