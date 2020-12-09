module Advent2020.Solutions.Tests.Day9

open Advent2020.Solutions.Day9
open FsUnit
open Xunit

module ``XMAS cracker`` =
    [<Fact>]
    let ``finds the first number that does not add up to the previous 5`` () =
        let input = [
            35 ;20; 15; 25; 47; 40; 62; 55; 65; 95; 102; 117; 150; 182; 127; 219; 299; 277; 309; 576
        ]
        let window = 5
        let expected = Some 127
        let result = input |> XmasCracker.findNonSumming window
        result |> should equal expected
