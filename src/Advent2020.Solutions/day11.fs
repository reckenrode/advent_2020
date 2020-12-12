module Advent2020.Solutions.Day11

open FSharpx.Result
open Utilities

let private rcFromList = function
| r::c::_ -> (r, c)
| _ -> failwith "expected two elements but got something else. This should not happen."

let mkFilter neighbors evaluateTally grid =
    let width = grid |> Array2D.width
    let height = grid |> Array2D.height
    Array2D.init height width <| fun r c ->
        let currentCell = grid.[r, c]
        if currentCell = '.'
        then currentCell
        else
            let occupied =
                neighbors (r, c) grid
                |> Seq.filter (fun (r, c) -> r >= 0 && c >= 0 && r < height && c < width)
                |> Seq.map (fun (r, c) -> grid.[r, c])
                |> Seq.fold (fun ocp cell -> if cell = '#' then ocp + 1 else ocp) 0
            evaluateTally occupied currentCell

let mkEvaluateTally target tally cell=
    match cell with
    | '#' when tally >= target -> 'L'
    | 'L' when tally = 0 -> '#'
    | _ -> cell

let neighbors (r, c) _ =
    Day1.enumerateNeighbors [r; c] |> Seq.map rcFromList

let nearbyFilter = mkFilter neighbors (mkEvaluateTally 4)

let rec private castRays (r, c) grid ((deltaR, deltaC) as v)  =
    let width = grid |> Array2D.width
    let height = grid |> Array2D.height
    let r = r + deltaR
    let c = c + deltaC
    if r >= 0 && r < height && c >= 0 && c < width && grid.[r, c] = '.'
    then castRays (r, c) grid v
    else (r, c)

let lineOfSightFilter =
    mkFilter
        (fun pos grid ->
            Day1.enumerateNeighbors [0; 0]
            |> Seq.map rcFromList
            |> Seq.map (castRays pos grid))
        (mkEvaluateTally 5)

let rec waitUntilStable f area =
    let result = area |> WaitingArea.applyRules f
    if result = area
    then result
    else result |> waitUntilStable f

let name = "day11"

let run (input: string, arg: string) =
    result {
        let! area = input |> WaitingArea.parse
        let filter = if arg = "los" then lineOfSightFilter else nearbyFilter
        let result = area |> waitUntilStable filter
        let occupiedChairs = string result |> Seq.filter ((=) '#') |> Seq.length
        return printfn $"There are {occupiedChairs} occupied seats."
    }
