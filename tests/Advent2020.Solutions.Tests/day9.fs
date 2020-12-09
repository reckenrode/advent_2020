module Advent2020.Solutions.Tests.Day9

open Advent2020.Solutions.Day9
open FsUnit
open Xunit

module ``XMAS cracker`` =
    [<Fact>]
    let ``finds the first number that does not add up to the previous 5`` () =
        let input = [
            35L; 20L; 15L; 25L; 47L; 40L; 62L; 55L; 65L; 95L; 102L;
            117L; 150L; 182L; 127L; 219L; 299L; 277L; 309L; 576L
        ]
        let window = 5
        let expected = Some 127L
        let result = input |> XmasCracker.findNonSumming window
        result |> should equal expected

    [<Fact>]
    let ``finds continuous sequences that add up to the bad number`` () =
        let input = [
            35L; 20L; 15L; 25L; 47L; 40L; 62L; 55L; 65L; 95L; 102L;
            117L; 150L; 182L; 127L; 219L; 299L; 277L; 309L; 576L
        ]
        let badNumber = 127L
        let expected = [15L; 25L; 47L; 40L]
        let result = input |> XmasCracker.findWeakSequence badNumber
        result |> should equal expected
