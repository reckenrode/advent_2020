open System

open Advent2020.Solutions

let usage () =
    printfn "usage: advent_2020 <day> <input>"
    printfn "\nDays:"
    let days = Inventory.solutions |> Map.toSeq |> Seq.map fst |> Seq.sort
    days |> Seq.iter (printfn "   %s")
    -1

let explainError (ex: exn) =
    printfn $"Encountered an error while parsing the input file: {ex.Message}"
    -1

let runSolution solution input part =
    solution (input, part)
    0

[<EntryPoint>]
let main argv =
    let day = argv |> Array.tryHead |> Option.bind (fun x -> Map.tryFind x Inventory.solutions)
    let input = argv |> Array.tryItem 1 |> Option.map Utilities.readFile
    let part = argv |> Array.tryItem 2 |> Option.orElse (Some "any")
    match day, input, part with
    | Some day, Some (Ok input), Some part -> runSolution day input part
    | _, Some (Error ex), _ -> explainError ex
    | _ -> usage ()
