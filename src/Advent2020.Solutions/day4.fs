module Advent2020.Solutions.Day4

open FParsec
open FSharpx.Option
open System.Globalization

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

type BirthYear = private BirthYear of uint

module BirthYear =
    let create str =
        maybe {
            let! x = Utilities.tryParse str
            let! year = if x >= 1920u && x <= 2002u then Some x else None
            return BirthYear year
        }

type IssueYear = private IssueYear of uint

module IssueYear =
    let create str =
        maybe {
            let! x = Utilities.tryParse str
            let! year = if x >= 2010u && x <= 2020u then Some x else None
            return IssueYear year
        }

type ExpirationYear = private ExpirationYear of uint

module ExpirationYear =
    let create str =
        maybe {
            let! x = Utilities.tryParse str
            let! year = if x >= 2020u && x <= 2030u then Some x else None
            return ExpirationYear year
        }

type EyeColor = Amber | Blue | Brown | Gray | Green | Hazel | Other

module EyeColor =
    let mapping = dict [
        ("amb", Amber)
        ("blu", Blue)
        ("brn", Brown)
        ("gry", Gray)
        ("grn", Green)
        ("hzl", Hazel)
        ("oth", Other)
    ]
    let create str = mapping |> FSharpx.Collections.Dictionary.tryFind str

type HairColor = private HairColor of byte * byte * byte

module HairColor =
    let convert (octet: array<char>) =
        let str = System.ReadOnlySpan octet
        System.Byte.Parse (str, NumberStyles.HexNumber, CultureInfo.InvariantCulture)

    let hexoctet = parray 2 (anyOf "0123456789abcdef") |>> convert
    let colortuple = tuple3 hexoctet hexoctet hexoctet
    let parser = Primitives.(>>.) (pchar '#') colortuple .>> eof

    let create str =
        match run parser str with
        | Success (color, _, _) -> Some (HairColor color)
        | _ -> None

type Height = private Height of byte * string

module Height =
    let parser = puint8 .>>. (pstring "cm" <|> pstring "in") .>> eof

    let create str =
        match run parser str with
        | Success ((hgt, "cm") as height, _, _) when hgt >= 150uy && hgt <= 193uy
            -> Some (Height height)
        | Success ((hgt, "in") as height, _, _) when hgt >= 59uy && hgt <= 76uy
            -> Some (Height height)
        | _ -> None

type PassportId = private PassportId of string

module PassportId =
    let parser = parray 9 digit .>> eof

    let create str =
        match run parser str with
        | Success (digits, _, _) -> Some (System.String.Concat digits |> PassportId)
        | _ -> None

type Passport = {
    byr: BirthYear
    iyr: IssueYear
    eyr: ExpirationYear
    hgt: Height
    hcl: HairColor
    ecl: EyeColor
    pid: PassportId
    cid: option<string>
}

module Passport =
    let ofRaw (raw: Map<string, string>) =
        maybe {
            let! byr = raw |> Map.tryFind "byr" >>= BirthYear.create
            let! iyr = raw |> Map.tryFind "iyr" >>= IssueYear.create
            let! eyr = raw |> Map.tryFind "eyr" >>= ExpirationYear.create
            let! hgt = raw |> Map.tryFind "hgt" >>= Height.create
            let! hcl = raw |> Map.tryFind "hcl" >>= HairColor.create
            let! ecl = raw |> Map.tryFind "ecl" >>= EyeColor.create
            let! pid = raw |> Map.tryFind "pid" >>= PassportId.create
            let cid = raw |> Map.tryFind "cid"
            return {
                byr = byr; iyr = iyr; eyr = eyr; hgt = hgt;
                hcl = hcl; ecl = ecl; pid = pid; cid = cid
            }
        }

let run (input: string, arg: string) =
    match input |> run passportList with
    | Failure (ex, _, _) -> printfn $"{ex}"
    | Success (passports, _, _) ->
        let validPassports =
            if arg = "validate_data"
            then passports |> List.map Passport.ofRaw |> List.choose id |> List.length
            else passports |> List.filter isValidPassport |> List.length
        printfn $"# valid passports: {validPassports}"
