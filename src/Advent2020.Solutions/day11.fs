module Advent2020.Solutions.Day11

open Utilities

let private rcFromList = function
| r::c::_ -> (r, c)
| _ -> failwith "expected two elements but got something else. This should not happen."

let nearbyFilter grid =
    let width = grid |> Array2D.width
    let height = grid |> Array2D.height
    let neighbors (r, c) =
        Day1.enumerateNeighbors [r; c]
        |> Seq.map rcFromList
        |> Seq.filter (fun (r, c) -> r >= 0 && c >= 0 && r < height && c < width)
        |> Seq.map (fun (r, c) -> grid.[r, c])
    Array2D.init height width <| fun r c ->
        let currentCell = grid.[r, c]
        if currentCell = '.'
        then currentCell
        else
            let occupied =
                neighbors (r, c)
                |> Seq.fold (fun ocp cell -> if cell = '#' then ocp + 1 else ocp) 0
            match currentCell with
            | '#' when occupied >= 4 -> 'L'
            | 'L' when occupied = 0 -> '#'
            | _ -> currentCell

let name = "day11"

// let run (input: string, arg: string) =
