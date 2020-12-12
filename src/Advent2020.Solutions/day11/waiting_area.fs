module Advent2020.Solutions.WaitingArea

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
