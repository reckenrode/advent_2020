module Advent2020.Solutions.Day10

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
        |> Seq.map (fun (lhs, rhs) -> rhs - lhs)
        |> Seq.countBy id
        |> Map.ofSeq
