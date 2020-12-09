module Advent2020.Solutions.Day7

open Utilities

module BagSolver =
    let canContain graph bag =
        let rec canContain' graph seen bag =
            let row = Array2D.rowSpan bag graph
            let (newSeen, _) = Span.fold (fun (s, idx) x ->
                if x <> 0
                then (s |> Set.add idx, idx + 1)
                else (s, idx + 1)) (Set.empty, 0) row
            if seen |> Set.isSubset newSeen
            then seen
            else
                let seen = seen |> Set.union newSeen
                newSeen |> Seq.map (canContain' graph seen) |> Set.unionMany
        canContain' graph Set.empty bag

let name = "day7"

let run (input: seq<string>, arg: string) =
    ()
