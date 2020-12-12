namespace Advent2020.Solutions

open FSharp.Core
open System.Reflection

module Inventory =
    let private isSolutionName (p: PropertyInfo) =
        (not <| isNull p) && p.PropertyType = typeof<string>

    let private isRunMethod (m: MethodInfo) =
        (not <| isNull m)
        && (m.GetParameters () |> Array.map (fun x -> x.ParameterType)) = [| typeof<seq<string>>; typeof<string> |]
        && m.ReturnType = typeof<System.Void>

    let private isRunMethodRaw (m: MethodInfo) =
        (not <| isNull m)
        && (m.GetParameters () |> Array.map (fun x -> x.ParameterType)) = [| typeof<string>; typeof<string> |]
        && m.ReturnType = typeof<System.Void>

    let private isRunMethodRawResult (m: MethodInfo) =
        (not <| isNull m)
        && (m.GetParameters () |> Array.map (fun x -> x.ParameterType)) = [| typeof<string>; typeof<string> |]
        && m.ReturnType = typeof<Result<FSharp.Core.Unit, string>>

    let private getName (p: PropertyInfo) =
        p.GetValue null :?> string

    let private makeRunFunc (m: MethodInfo) =
        fun (path: string, arg: string) ->
           Utilities.readFile path
           |> Result.map (fun lines -> m.Invoke (null, [| lines; arg |]) :?> unit)

    let private makeRunFuncRaw (m: MethodInfo) =
        fun (path: string, arg: string) ->
           Utilities.read path
           |> Result.map (fun input -> m.Invoke (null, [| input; arg |]) :?> unit)

    let private makeRunFuncRawResult (m: MethodInfo) =
        fun (path: string, arg: string) ->
           Utilities.read path
           |> Result.map (fun input ->
                match m.Invoke (null, [| input; arg |]) :?> Result<unit, string> with
                | Ok result -> result
                | Error ex -> printfn $"{ex}")

    let private solutionModule (t: System.Type) =
        let bindingFlags = BindingFlags.Public ||| BindingFlags.Static
        match (t.GetProperty "name", t.GetMethod ("run", bindingFlags)) with
        | (name, run) when isSolutionName name && isRunMethod run ->
            Some (getName name, makeRunFunc run)
        | (name, run) when isSolutionName name && isRunMethodRaw run ->
            Some (getName name, makeRunFuncRaw run)
        | (name, run) when isSolutionName name && isRunMethodRawResult run ->
            Some (getName name, makeRunFuncRawResult run)
        | _ -> None

    let solutions: Map<string, string * string -> Result<unit, exn>> =
        let assembly = Assembly.GetExecutingAssembly ()
        assembly.GetTypes ()
        |> Array.choose solutionModule
        |> Map.ofArray
