module Advent2020.Solutions.Tests.Day7

open Advent2020.Solutions.Day7
open FsUnit
open Xunit

module ``the bag solver`` =
    [<Fact>]
    let ``when finding bags it the one that can hold my bag`` () =
        let input = array2D [
            [ 0; 0; 0 ]
            [ 1; 0; 0 ]
            [ 0; 0; 0 ]
        ]
        let mybag = 1
        let expected = set [ 0 ]
        let foundBags = BagSolver.canContain input mybag
        foundBags |> should equal expected

    [<Fact>]
    let ``when finding bags it finds bags inside bags that can hold my bag`` () =
        let input = array2D [
            [ 0; 0; 0; 0; 0; 0; 0; 0; 0 ]
            [ 1; 0; 3; 0; 0; 0; 0; 0; 0 ]
            [ 0; 0; 0; 0; 0; 0; 0; 0; 0 ]
            [ 2; 0; 4; 0; 0; 0; 0; 0; 0 ]
            [ 0; 1; 0; 2; 0; 0; 0; 0; 0 ]
            [ 0; 0; 0; 0; 1; 0; 0; 0; 0 ]
            [ 0; 0; 0; 0; 2; 0; 0; 0; 0 ]
            [ 0; 0; 0; 0; 0; 4; 6; 0; 0 ]
            [ 0; 0; 0; 9; 0; 3; 5; 0; 0 ]
        ]
        let mybag = 4
        let expected = set [ 0; 1; 2; 3 ]
        let foundBags = BagSolver.canContain input mybag
        foundBags |> should equal expected
