module Advent2020.Solutions.Day2

open FParsec

type Policy =  Policy of min: uint * max: uint * ch: char with
    member self.IsValid password =
        match self with
        | Policy (min, max, ch) ->
            let numOccurences = password |> Seq.filter ((=) ch) |> Seq.length |> uint
            numOccurences >= min && numOccurences <= max

module Policy =
    let isValid password (policy: Policy) = policy.IsValid password

type PasswordInfo = private PasswordInfo of Policy * string

module PasswordInfo =
    let policy = function PasswordInfo (policy, _) -> policy
    let password = function PasswordInfo (_, password) -> password
    let hasValidPassword pinfo =
        let policy = pinfo |> policy
        let passwd = pinfo |> password
        policy |> Policy.isValid passwd

let private policyParser =
    tuple3 (puint32 .>> pchar '-') (puint32 .>> spaces1) (asciiLower .>> pchar ':') |>> Policy

let private passwordParser =
    spaces1 >>. (many1 (satisfy (not << System.Char.IsWhiteSpace))) |>> System.String.Concat

let private parser = tuple2 policyParser passwordParser |>> PasswordInfo

let parse input =
    match run parser input with
    | Success (password, _, _) -> Some password
    | Failure _ -> None

let name = "day2"

let run (input: seq<string>, arg: string) =
    match input |> Seq.map parse |> Utilities.liftOption with
    | None -> printfn "Error: input file contains invalid password entries"
    | Some passwords ->
        let valid = passwords |> Seq.filter PasswordInfo.hasValidPassword |> Seq.length
        printfn $"# valid passwords: {valid}"
