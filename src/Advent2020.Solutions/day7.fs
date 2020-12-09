module Advent2020.Solutions.Day7

open FParsec
open FSharpx.Option
open Utilities

module BagSolver =
    let private setOfIndices (s, idx) x =
        if x <> 0
        then (s |> Set.add idx, idx + 1)
        else (s, idx + 1)

    let canContain graph bag =
        let rec canContain' graph seen bag =
            let row = Array2D.rowSpan bag graph
            let (newSeen, _) = Span.fold setOfIndices (Set.empty, 0) row
            if seen |> Set.isSubset newSeen
            then seen
            else
                let seen = seen |> Set.union newSeen
                newSeen |> Seq.map (canContain' graph seen) |> Set.unionMany
        canContain' graph Set.empty bag

type Rules = private Rules of array<string> * int [,]

module Rules =
    let private parser: Parser<string * list<uint32 * string>, unit> =
        let bagNameEnd = pstring " bags contain "
        let initialBagName = manyCharsTill (letter <|> pchar ' ') (followedBy bagNameEnd)
        let bagOrBags = pstring " bags" <|> pstring " bag"
        let bagName = manyCharsTill (letter <|> pchar ' ') (followedBy bagOrBags)
        let bagWithQuantity = puint32 .>> pchar ' ' .>>. bagName .>> bagOrBags
        let noBags = pstring "no other bags" |>> (fun _ -> [])
        let bagSequence = (attempt noBags) <|> sepBy bagWithQuantity (pstring ", ")
        initialBagName .>> bagNameEnd .>>. bagSequence .>> (pchar '.') .>> eof

    let private parseLine input =
        match input |> run parser with
        | Success (rule, _, _) -> Some rule
        | Failure (ex, _, _) -> printfn $"{ex}"; None

    let private buildGraph size (rules: list<string * list<uint32 * string>>) =
        let indices = rules |> Seq.map fst |> Array.ofSeq
        let idxMap = indices |> Seq.mapi (fun idx x -> (x, idx)) |> Map.ofSeq
        let mutable graph = Array2D.init size size (fun _ _ -> 0)
        rules |> Seq.iter (fun (container, containees) ->
            containees |> Seq.iter (fun (quantity, containee) ->
                let containerIdx = idxMap |> Map.find container
                let containeeIdx = idxMap |> Map.find containee
                graph.[containeeIdx, containerIdx] <- (int quantity)))
        (indices, graph)

    let parse input =
        maybe {
            let graphSize = input |> Seq.length
            let! parsedRules = input |> Seq.map parseLine |> List.ofSeq |> sequence
            return Rules (parsedRules |> buildGraph graphSize)
        }

    let outermostContainingBags bag (Rules (indices, graph)) =
        let bagIdx = indices |> Array.findIndex ((=) bag)
        BagSolver.canContain graph bagIdx |> Set.map (fun idx -> indices |> Array.item idx)

let name = "day7"

let run (input: seq<string>, arg: string) =
    match Rules.parse input with
    | None -> printfn "error parsing bags"
    | Some rules ->
        let bags = rules |> Rules.outermostContainingBags "shiny gold"
        printfn $"# bags that can contain a shiny gold bag: {bags |> Set.count}"
