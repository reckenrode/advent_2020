module Advent2020.Solutions.Tests.Generators

open FsCheck

let lowerAscii = Gen.choose (int 'a', int 'z') |> Gen.map char |> Arb.fromGen

let policyEnding = Gen.frequency [(9, Arb.generate<char>); (1, Gen.constant ':')] |> Arb.fromGen

let password =
    gen {
        let! length = Gen.choose (1, 512)
        let nonSpaceChars = Arb.generate<char> |> Gen.filter (not << System.Char.IsWhiteSpace)
        return! nonSpaceChars
        |> Gen.arrayOfLength length
        |> Gen.map System.String.Concat
    } |> Arb.fromGen

let spaces =
    gen {
        let! length = Gen.choose (1, 32)
        return! Gen.constant ' '
        |> Gen.arrayOfLength length
        |> Gen.map System.String.Concat
    } |> Arb.fromGen
