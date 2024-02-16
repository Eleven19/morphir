﻿namespace Morphir.Hosting

open Morphir.Host


type Host(configuration: HostConfiguration) =
    static member DefaultConfiguration = HostConfiguration.Default
    static Default = Host(Host.DefaultConfiguration)

    member __.RunAsync() =
        async {
            configuration.Extensions.ExtensionsSearchLocations
            |> List.iter (fun location -> printfn "Extension Location: %A" location)
        }

and HostBuilder(configuration: HostConfiguration) =
    member __.Build() = Host(configuration)

module Host =

    let defaultConfiguration = Host.DefaultConfiguration
    let Default = Host.Default
