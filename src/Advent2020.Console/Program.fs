open System

open Advent2020.Solutions

let usage () =
    printfn "usage: advent_2020 <day> <input>"
    printfn "\nDays:"
    for day in Inventory.solutions |> Map.toSeq |> Seq.map fst do
        printfn $"   {day}"
    -1

[<EntryPoint>]
let main argv =
    let day = argv |> Array.tryHead |> Option.bind (fun x -> Map.tryFind x Inventory.solutions)
    let input = argv |> Array.tryItem 1
    match (day, input) with
    | Some day, Some input ->
        printfn $"Got: {day} with input: {input}"
        0
    | _ -> usage ()
