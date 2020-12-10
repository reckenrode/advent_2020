module Advent2020.Solutions.Tests.Day10

open Advent2020.Solutions.Day10
open FsUnit
open Xunit

module ``the adapter analyzer`` =
    [<Fact>]
    let ``arranges the adapters in a chain including all adapters`` () =
        let input = [ 16; 10; 15; 5; 1; 11; 7; 19; 6; 12; 4 ]
        let expected = Some [ 1; 4; 5; 6; 7; 10; 11; 12; 15; 16; 19 ]
        let result = AdapterAnalyzer.createChain 0 input
        result |> should equal expected

    [<Fact>]
    let ``calculates the differences between each adapter in a chain`` () =
        let input = [ 1; 4; 5; 6; 7; 10; 11; 12; 15; 16; 19 ]
        let expected = Map.ofList [ (1, 7); (3, 5) ]
        let result = AdapterAnalyzer.findDifferences 0 input
        result |> should equal expected

    [<Fact>]
    let ``counts the number of ways to arrange the adapters`` () =
        let input = [ 16; 10; 15; 5; 1; 11; 7; 19; 6; 12; 4 ]
        let expected = 8L
        let result = AdapterAnalyzer.countArrangements 0 input
        result |> should equal expected
