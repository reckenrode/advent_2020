module Advent2020.Solutions.Day6

open FParsec
open FSharpx.Option

let name = "day6"

type Group = list<Responses>
and Responses = Set<Answers>
and Answers = private Answers of char

module Answers =
    let create = function
    | ch when ch >= 'a' && ch <= 'z' -> Some (Answers ch)
    | _ -> None

module Responses =
    let create lst =
        maybe {
            let! lst = lst |> List.map Answers.create |> sequence
            return Set.ofList lst
        }

    let parser<'ex> : Parser<option<Responses>, 'ex> =
        let questions = asciiLower
        many1 questions |>> create

module Group =
    let countYesAnswers: Group -> int = Set.unionMany >> Set.count
    let countCommonYesAnswers: Group -> int = Set.intersectMany >> Set.count

    let parser<'ex> : Parser<option<Group>, 'ex> =
        sepEndBy1 Responses.parser newline |>> sequence

let groupsParser<'ex> : Parser<option<list<Group>>, 'ex> =
    sepBy1 Group.parser newline |>> sequence .>> eof

let run (input: string, arg: string) =
    match input |> run groupsParser with
    | Failure (ex, _, _) -> printfn $"{ex}"
    | Success (Some groups, _, _) ->
        let countf = if arg = "every" then Group.countCommonYesAnswers else Group.countYesAnswers
        let n = groups |> List.map countf |> List.sum
        printfn $"Yes answers for {arg}one: {n}"
    | Success (None, _, _) -> printfn "Some of the groups contained invalid data."
