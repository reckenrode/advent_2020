module Advent2020.Solutions.Day1

open FsRandom

let name = "day1"

let targetNumber = 2020

let distance target (ps: list<int>) (data: array<int>) =
    let sum = ps |> List.fold (fun acc x -> acc + data.[x]) 0
    (target - sum) |> System.Math.Abs

let enumerateNeighbors ps =
    ps
    |> List.map (fun idx -> [idx - 1; idx; idx + 1])
    |> Utilities.pairs
    |> Seq.filter (fun p -> p <> ps)

let isInRange bot top x =
    x >= bot && x < top

let rec climb target pos currentDistance data =
    random {
        if currentDistance = 0
        then return Some (pos |> List.map (fun x -> data |> Array.item x))
        else
            let maxPos = data |> Array.length
            let isInRange = isInRange 0 maxPos
            let possibilities =
                enumerateNeighbors pos
                |> Seq.filter (List.forall isInRange)
                |> Seq.filter (Utilities.allUnique)
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

let rec uniqueRandomCreate size gen =
    random {
        let! result = List.randomCreate size gen
        if result |> Utilities.allUnique
        then return result
        else return! uniqueRandomCreate size gen
    }

let rec findSolution target numbers size =
    random {
        let maxPos = (numbers |> Array.length) - 1
        let! start = uniqueRandomCreate size (Statistics.uniformDiscrete (0, maxPos))
        match! numbers |> climb target start (distance target start numbers) with
        | Some result -> return result
        | None -> return! findSolution target numbers size
    }

let run (input: seq<string>, part: string) =
    match input |> Seq.map Utilities.tryParse |> Utilities.liftOption with
    | None -> printfn "Error: input file not in expected format (a list of integers)."
    | Some numbers ->
        let size = if part = "part2" then 3 else 2
        let numbers = numbers |> List.sort |> Array.ofList
        let rng = Utility.createRandomState ()
        let result = rng |> Random.get (findSolution targetNumber numbers size)
        let exp = System.String.Join ("×", result)
        printfn $"{exp} = {result |> List.fold (*) 1}"
