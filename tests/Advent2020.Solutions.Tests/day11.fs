module Advent2020.Solutions.Tests.Day11

open Advent2020.Solutions
open FSharpx.Result
open FsUnit
open Xunit

module ``Waiting Room Filters`` =
    [<Fact>]
    let hmm () =
        true |> should equal true

    [<Fact>]
    let ``an empty seat becomes occupied when no occupied seats are adjacent`` () =
        result {
            let! waitingArea = WaitingArea.parse "L.LL\nLLLL"
            let expected = "#.##\n####"
            let result = waitingArea |> WaitingArea.applyRules Day11.nearbyFilter
            return (string result) |> should equal expected
        }

    [<Fact>]
    let ``an occupied seat becomes empty when four or more adjacent seats are occupied`` () =
        result {
            let! waitingArea = WaitingArea.parse "#.##\n####"
            let expected = "#.L#\n#LL#"
            let result = waitingArea |> WaitingArea.applyRules Day11.nearbyFilter
            return (string result) |> should equal expected
        }
