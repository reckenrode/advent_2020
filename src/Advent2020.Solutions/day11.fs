module Advent2020.Solutions.Day11

open FParsec
open FSharpx
open Utilities

type WaitingArea = private WA of char [,] with
    member self.Width = let (WA w) = self in w |> Array2D.width
    member self.Height = let (WA w) = self in w |> Array2D.height
    override self.ToString () =
        let (WA w) = self
        let rec renderGrid row acc =
            if row < 0
            then acc |> String.concat "\n"
            else
                let str = System.String (Array2D.rowSpan row w)
                renderGrid (row - 1) (str::acc)
        renderGrid (self.Height - 1) []

module WaitingArea =
    let width (w: WaitingArea) = w.Width
    let height (w: WaitingArea) = w.Height
    let applyRules f (WA w) = WA (f w)
    let parse str =
        let tile = attempt (pchar '.') <|> attempt (pchar '#') <|> pchar 'L'
        let row = many1Chars tile// |>> (fun str -> (str, str |> String.length))
        let grid = sepEndBy1 row newline
        let parser = grid .>> eof
        match str |> run parser with
        | Failure (ex, _, _) -> Result.Error ex
        | Success (lst, _, _) ->
            let result = Option.maybe {
                let! firstRow = lst |> List.tryHead
                let width = firstRow |> String.length
                return
                    match lst |> List.partition (String.length >> ((=) width)) with
                    | (good, []) ->
                        let good = good |> Array.ofList
                        let height = good |> Array.length
                        let arr = Array2D.init height width (fun r c -> (good |> Array.item r).[c])
                        Result.Ok (WA arr)
                    | (good, bad::_) ->
                        let goodLines = good |> String.concat "\n"
                        let caretOffset = bad |> String.length
                        let caret = sprintf "%*c" caretOffset '^'
                        Result.Error ([
                            "Error: grid contains uneven lines."
                            $"{goodLines}"
                            $"{bad}"
                            $"{caret}"
                        ] |> String.concat "\n")
            }
            result |> Option.getOrElse (Result.Ok (WA (Array2D.init 0 0 (fun _ _ -> '0'))))

let private rcFromList = function
| r::c::_ -> (r, c)
| _ -> failwith "expected two elements but got something else. This should not happen."

let fourNearFilter grid =
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
