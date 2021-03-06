module Advent2020.Solutions.Day1

open FsRandom

let name = "day1"

let targetNumber = 2020L

let distance target ps (data: array<int64>) =
    let sum = ps |> List.fold (fun acc x -> acc + data.[x]) 0L
    abs (target - sum)

let enumerateNeighbors ps =
    ps
    |> List.map (fun idx -> [idx - 1; idx; idx + 1])
    |> Utilities.pairs
    |> Seq.filter (fun p -> p <> ps)

let isInRange bot top x =
    x >= bot && x < top

let rec climb dist pred target pos currentDistance data =
    random {
        if currentDistance = 0L
        then return Some pos
        else
            let maxPos = data |> Array.length
            let isInRange = isInRange 0 maxPos
            let possibilities =
                enumerateNeighbors pos
                |> Seq.filter (List.forall isInRange)
                |> Seq.filter pred
                |> Seq.map (fun pos -> (pos, dist target pos data))
                |> Seq.filter (fun (_, d) -> d < currentDistance)
                |> Array.ofSeq
            if possibilities |> Array.length = 0
            then return None
            else
                let! candidate = possibilities |> Array.sampleOne
                let newPos, newDistance = candidate
                return! data |> climb dist pred target newPos newDistance
    }

let rec uniqueRandomCreate size gen =
    random {
        let! result = List.randomCreate size gen
        if result |> Utilities.allUnique
        then return result
        else return! uniqueRandomCreate size gen
    }

let rec findSolutionGeneric dist pred target numbers size =
    random {
        let maxPos = (numbers |> Array.length) - 1
        let! start = uniqueRandomCreate size (Statistics.uniformDiscrete (0, maxPos))
        match! numbers |> climb dist pred target start (dist target start numbers) with
        | Some result -> return result
        | None -> return! findSolutionGeneric dist pred target numbers size
    }

let findSolution target numbers size =
    random {
        let! pos = findSolutionGeneric distance Utilities.allUnique target numbers size
        return pos |> List.map (fun x -> numbers |> Array.item x)
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
        printfn $"{exp} = {result |> List.fold (*) 1L}"
