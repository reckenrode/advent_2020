module Advent2020.Solutions.Day2

open FParsec

type Policy =
    | Policy of min: uint * max: uint * ch: char

type Password = private Password of Policy * string

module Password =
    let policy = function Password (policy, _) -> policy
    let password = function Password (_, password) -> password

let private policyParser =
    tuple3 (puint32 .>> pchar '-') (puint32 .>> spaces1) (asciiLower .>> pchar ':') |>> Policy

let private passwordParser =
    spaces1 >>. (many1 (satisfy (not << System.Char.IsWhiteSpace))) |>> System.String.Concat

let private parser = tuple2 policyParser passwordParser |>> Password

let parse input =
    match run parser input with
    | Success (password, _, _) -> Some password
    | Failure _ -> None
