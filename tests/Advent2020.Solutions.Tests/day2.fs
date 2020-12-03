module Advent2020.Solutions.Tests.Day2

open Advent2020.Solutions.Day2

open Advent2020.Solutions.Tests.Generators

open FsCheck
open FsCheck.Xunit
open FsUnit
open Xunit

module ``Password Parser`` =
    [<Property>]
    let ``the policy parser accepts leading digits`` (digit: uint) =
        let input = $"{digit}-2 a: abcd"
        let result = parse input
        let minChars = result |> Option.map (fun o ->
            match o |> PasswordInfo.policy with
            | OldPolicy (minChars, _, _) -> minChars
            | _ -> failwith "n/a")
        minChars |> should equal (Some digit)

    [<Property>]
    let ``the policy parser accepts trailing digits`` (digit: uint) =
        let input = $"2-{digit} b: efgh"
        let result = parse input
        let maxChars = result |> Option.map (fun o ->
            match o |> PasswordInfo.policy with
            | OldPolicy (_, maxChars, _) -> maxChars
            | _ -> failwith "n/a")
        maxChars |> should equal (Some digit)

    [<Property>]
    let ``the policy parser accepts space after the last digit and before the character`` () =
        Prop.forAll spaces <| fun sp ->
            let input = $"0-3{sp}b: zxcv"
            let result = parse input
            let policy = result |> Option.map PasswordInfo.policy
            policy |> should equal (Some (OldPolicy (0u, 3u, 'b')))

    [<Fact>]
    let ``the policy parser rejects a lack of space between the last digit and character`` () =
        let input = "0-3b: zxcv"
        let result = parse input
        result |> should equal None

    [<Property>]
    let ``the policy parser accepts a single character`` () =
        Prop.forAll lowerAscii <| fun ch ->
            let input = $"3-4 {ch}: ijkl"
            let result = parse input
            let mandatoryCharacter = result |> Option.map (fun o ->
                match o |> PasswordInfo.policy with
                | OldPolicy (_, _, ch) -> ch
                | _ -> failwith "n/a")
            mandatoryCharacter |> should equal (Some ch)

    [<Property>]
    let ``the policy parser rejects space before the dash`` () =
        Prop.forAll spaces <| fun sp ->
            let input = $"5{sp}-6 c: mnop"
            let result = parse input
            result |> should equal None

    [<Property>]
    let ``the policy parser rejects space after the dash`` () =
        Prop.forAll spaces <| fun sp ->
            let input = $"7-{sp}8 c: qrst"
            let result = parse input
            result |> should equal None

    [<Property>]
    let `` the policy parser requires the policy end with a colon`` () =
        Prop.forAll policyEnding <| fun ch ->
            let input = $"9-10 d{ch} mnop"
            let result = parse input
            let policy = result |> Option.map PasswordInfo.policy
            if ch = ':'
            then policy |> should equal (Some (OldPolicy (min=9u, max=10u, ch='d')))
            else policy |> should equal None

    [<Property>]
    let ``the password parser ignores the leading whitespace after the policy ending`` () =
        Prop.forAll spaces <| fun sp ->
            let input = $"11-12 e:{sp}qrst"
            let result = parse input
            let password = result |> Option.map PasswordInfo.password
            password |> should equal (Some "qrst")

    [<Fact>]
    let ``the password parser requires at least one space after the policy ending`` () =
        let input = "867-5301 j:80’ssong"
        let result = parse input
        result |> should equal None

    [<Property>]
    let ``the password parser parses everything else as the password`` () =
        Prop.forAll password <| fun pw ->
            let input = $"13-14 f: {pw}"
            let result = parse input
            let password = result |> Option.map PasswordInfo.password
            password |> should equal (Some pw)

module ``Password Validation`` =
    [<Fact>]
    let ``flags passwords with at least the required number of characters as valid`` () =
        let input = "1-10 a: abcdefg"
        let result = parse input
        let isValid = result |> Option.map PasswordInfo.hasValidPassword
        isValid |> should equal (Some true)

    [<Fact>]
    let ``flags passwords with no more than the required number of characters as valid`` () =
        let input = "1-10 a: abacadaeafagaaaa"
        let result = parse input
        let isValid = result |> Option.map PasswordInfo.hasValidPassword
        isValid |> should equal (Some true)

    [<Fact>]
    let ``flags passwords with not enough required characters as valid`` () =
        let input = "1-10 q: abacadaeafagaaaa"
        let result = parse input
        let isValid = result |> Option.map PasswordInfo.hasValidPassword
        isValid |> should equal (Some false)

    [<Fact>]
    let ``flags passwords with too many required characters as valid`` () =
        let input = "1-5 a: abacadaeafagaaaa"
        let result = parse input
        let isValid = result |> Option.map PasswordInfo.hasValidPassword
        isValid |> should equal (Some false)

    [<Fact>]
    let ``new policy flags passwords with the first index as valid`` () =
        let input = "1-10 a: abacadaeafagaaaa"
        let result = parseWithPolicy NewPolicy input
        let isValid = result |> Option.map PasswordInfo.hasValidPassword
        isValid |> should equal (Some true)

    [<Fact>]
    let ``new policy flags passwords with the second index as valid`` () =
        let input = "1-10 f: abacadaeafagaaaa"
        let result = parseWithPolicy NewPolicy input
        let isValid = result |> Option.map PasswordInfo.hasValidPassword
        isValid |> should equal (Some true)

    [<Fact>]
    let ``new policy flags passwords with the both indices as invalid`` () =
        let input = "1-3 a: abacadaeafagaaaa"
        let result = parseWithPolicy NewPolicy input
        let isValid = result |> Option.map PasswordInfo.hasValidPassword
        isValid |> should equal (Some false)

    [<Fact>]
    let ``new policy flags passwords with the neither index as invalid`` () =
        let input = "1-10 z: abacadaeafagaaaa"
        let result = parseWithPolicy NewPolicy input
        let isValid = result |> Option.map PasswordInfo.hasValidPassword
        isValid |> should equal (Some false)
