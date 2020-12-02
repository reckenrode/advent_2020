open System

open Advent2020.Solutions

let usage () =
    printfn "usage: advent_2020 <day> <input>"
    printfn "\nDays:"
    for day in Inventory.solutions |> Map.toSeq |> Seq.map fst |> Seq.sort do
        printfn $"   {day}"
    -1

let explainError (ex: exn) =
    printfn $"Encountered an error while parsing the input file: {ex.Message}"
    -1

let runSolution solution input =
    printfn $"{solution input}"
    0

[<EntryPoint>]
let main argv =
    let day = argv |> Array.tryHead |> Option.bind (fun x -> Map.tryFind x Inventory.solutions)
    let input = argv |> Array.tryItem 1 |> Option.map Utilities.readFile
    match (day, input) with
    | Some day, Some (Ok input) -> runSolution day input
    | Some day, Some (Error ex) -> explainError ex
    | _ -> usage ()
