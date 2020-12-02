namespace Advent2020.Solutions

open FSharp.Core
open System.Reflection

module Inventory =
    let private isSolutionName (p: PropertyInfo) =
        (not <| isNull p) && p.PropertyType = typeof<string>

    let private isRunMethod (m: MethodInfo) =
        (not <| isNull m)
        && (m.GetParameters () |> Array.map (fun x -> x.ParameterType)) = [| typeof<seq<string>> |]
        && m.ReturnType = typeof<System.Void>

    let getName (p: PropertyInfo) =
        p.GetValue null :?> string

    let makeRunFunc (m: MethodInfo) =
        fun (x: seq<string>) -> m.Invoke (null, [| x |]) :?> unit

    let private solutionModule (t: System.Type) =
        let bindingFlags = BindingFlags.Public ||| BindingFlags.Static
        match (t.GetProperty "name", t.GetMethod ("run", bindingFlags)) with
        | (name, run) when isSolutionName name && isRunMethod run ->
            Some (getName name, makeRunFunc run)
        | _ -> None

    let solutions: Map<string, seq<string> -> unit> =
        let assembly = Assembly.GetExecutingAssembly ()
        assembly.GetTypes ()
        |> Array.choose solutionModule
        |> Map.ofArray
