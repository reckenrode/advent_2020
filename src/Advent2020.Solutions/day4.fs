module Advent2020.Solutions.Day4

open FParsec

let name = "day4"

let passportList =
    let mkPassportField = function
    | [] -> failwith "error: passports have fields"
    | (x::xs) -> xs |> List.fold (fun parser field -> parser <|> pstring field) (pstring x)
    let mkList x xs = x::xs
    let passportField = mkPassportField [ "byr"; "iyr"; "eyr"; "hgt"; "hcl"; "ecl"; "pid"; "cid" ]
    let passportData = many1 (satisfy (System.Char.IsWhiteSpace >> not)) |>> System.String.Concat
    let passportElement = tuple2 (passportField .>> pchar ':') passportData
    let passportSeparator = pchar ' ' <|> newline
    let passport = pipe2 passportElement (many (passportSeparator >>? passportElement)) mkList |>> Map.ofList .>> newline
    sepEndBy1 passport newline .>> eof

let allPassportFields = [ "byr"; "iyr"; "eyr"; "hgt"; "hcl"; "ecl"; "pid" ] |> List.sort

let isValidPassport p =
    let fields = p |> Map.toSeq |> Seq.map fst |> Seq.filter ((<>) "cid") |> Seq.sort |> Seq.toList
    fields = allPassportFields

let run (input: string, arg: string) =
    match input |> run passportList with
    | Failure (ex, _, _) -> printfn $"{ex}"
    | Success (passports, _, _) ->
        let validPassports = passports |> List.filter isValidPassport
        printfn $"# valid passports: {validPassports |> List.length}"
