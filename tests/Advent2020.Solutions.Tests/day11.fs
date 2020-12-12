module Advent2020.Solutions.Tests.Day11

open Advent2020.Solutions
open FSharpx.Result
open FsUnit
open Xunit

module ``Waiting Room Filters`` =
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

module ``Line of Sight Filters`` =
    [<Fact>]
    let ``an empty seat becomes occupied when no occupied seats are adjacent`` () =
        result {
            let! waitingArea = WaitingArea.parse "L.LL\nLLLL"
            let expected = "#.##\n####"
            let result = waitingArea |> WaitingArea.applyRules Day11.lineOfSightFilter
            return (string result) |> should equal expected
        }

    [<Fact>]
    let ``an occupied seat becomes empty when four or more adjacent seats are occupied`` () =
        result {
            let! waitingArea = WaitingArea.parse "####\n####"
            let expected = "#LL#\n#LL#"
            let result = waitingArea |> WaitingArea.applyRules Day11.lineOfSightFilter
            return (string result) |> should equal expected
        }

module ``Waiting Around`` =
    [<Fact>]
    let ``eventually stabilizes when iteratively running the filter`` () =
        result {
            let initialRoom = "\
                L.LL.LL.LL\n\
                LLLLLLL.LL\n\
                L.L.L..L..\n\
                LLLL.LL.LL\n\
                L.LL.LL.LL\n\
                L.LLLLL.LL\n\
                ..L.L.....\n\
                LLLLLLLLLL\n\
                L.LLLLLL.L\n\
                L.LLLLL.LL"
            let expectedContents = "\
                #.#L.L#.##\n\
                #LLL#LL.L#\n\
                L.#.L..#..\n\
                #L##.##.L#\n\
                #.#L.LL.LL\n\
                #.#L#L#.##\n\
                ..L.L.....\n\
                #L#L##L#L#\n\
                #.LLLLLL.L\n\
                #.#L#L#.##"
            let! waitingArea = WaitingArea.parse initialRoom
            let result = waitingArea |> Day11.waitUntilStable Day11.nearbyFilter
            return (string result) |> should equal expectedContents
        }
