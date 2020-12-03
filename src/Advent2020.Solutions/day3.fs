module Advent2020.Solutions.Day3

let name = "day3"

let advance dx dy = Utilities.trySkip dy >> Seq.map (Seq.skip dx)

let tallyTree = function
| '#' -> 1
| _ -> 0

let countTrees xs dx dy =
    let rec countTrees' n map =
        if map |> Seq.isEmpty
        then n
        else
            let pos = map |> Seq.head |> Seq.head
            let newMap = map |> advance dx dy
            countTrees' (n + tallyTree pos) newMap
    countTrees' 0 (xs |> advance dx dy)

let run (input: seq<string>, arg: string) =
    let (dx, dy) =
        match arg.Split(',') |> Array.map Utilities.tryParse with
        | [| Some dx; Some dy |] -> dx, dy
        | _ -> 3, 1
    let map = input |> Seq.map Utilities.cycle
    let nTrees = countTrees map dx dy
    printfn $"Santa encountered {nTrees} trees."
