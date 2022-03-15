module Morphir.IR.Repo exposing (..)

import Dict exposing (Dict)
import Morphir.Dependency.DAG as DAG exposing (DAG)
import Morphir.IR.AccessControlled as AccessControlled exposing (AccessControlled, public)
import Morphir.IR.Distribution exposing (Distribution(..))
import Morphir.IR.Documented as Documented
import Morphir.IR.FQName exposing (FQName)
import Morphir.IR.Module as Module exposing (ModuleName)
import Morphir.IR.Name exposing (Name)
import Morphir.IR.Package as Package exposing (PackageName)
import Morphir.IR.Type as Type exposing (Type)
import Morphir.Value.Native as Native
import Set exposing (Set)


type alias Repo =
    { packageName : PackageName
    , dependencies : Dict PackageName (Package.Definition () (Type ()))
    , modules : Dict ModuleName (AccessControlled (Module.Definition () (Type ())))
    , moduleDependencies : DAG ModuleName
    , nativeFunctions : Dict FQName Native.Function
    }


type alias SourceCode =
    String


type alias Errors =
    List Error


type Error
    = ModuleNotFound ModuleName
    | ModuleHasDependents ModuleName (Set ModuleName)
    | ModuleAlreadyExist ModuleName
    | TypeAlreadyExist Name


{-| Creates a repo from scratch when there is no existing IR.
-}
empty : PackageName -> Repo
empty packageName =
    { packageName = packageName
    , dependencies = Dict.empty
    , modules = Dict.empty
    , nativeFunctions = Dict.empty
    , moduleDependencies = DAG.empty
    }


{-| Creates a repo from an existing IR.
-}
fromDistribution : Distribution -> Result Errors Repo
fromDistribution distro =
    case distro of
        Library packageName _ packageDef ->
            packageDef.modules
                |> Dict.toList
                |> List.foldl
                    (\( moduleName, accessControlledModuleDef ) repoResultSoFar ->
                        repoResultSoFar
                            |> Result.andThen
                                (\repoSoFar ->
                                    repoSoFar
                                        |> insertModule moduleName accessControlledModuleDef.value
                                )
                    )
                    (Ok (empty packageName))


{-| Creates a distribution from an existing repo
-}
toDistribution : Repo -> Distribution
toDistribution repo =
    Library repo.packageName
        (repo.dependencies
            |> Dict.map (always Package.definitionToSpecification)
        )
        { modules =
            repo.modules
        }


{-| Adds native functions to the repo. For now this will be mainly used to add `SDK.nativeFunctions`.
-}
mergeNativeFunctions : Dict FQName Native.Function -> Repo -> Result Errors Repo
mergeNativeFunctions newNativeFunction repo =
    Ok
        { repo
            | nativeFunctions =
                repo.nativeFunctions
                    |> Dict.union newNativeFunction
        }


{-| Insert a module if it's not in the repo yet.
-}
insertModule : ModuleName -> Module.Definition () (Type ()) -> Repo -> Result Errors Repo
insertModule moduleName moduleDef repo =
    let
        validationErrors : Maybe Errors
        validationErrors =
            case repo.modules |> Dict.get moduleName of
                Just _ ->
                    Just [ ModuleAlreadyExist moduleName ]

                Nothing ->
                    Nothing
    in
    validationErrors
        |> Maybe.map Err
        |> Maybe.withDefault
            (Ok
                { repo
                    | modules =
                        repo.modules
                            |> Dict.insert moduleName (AccessControlled.private moduleDef)
                }
            )


deleteModule : ModuleName -> Repo -> Result Errors Repo
deleteModule moduleName repo =
    let
        validationErrors : Maybe Errors
        validationErrors =
            case repo.modules |> Dict.get moduleName of
                Nothing ->
                    Just [ ModuleNotFound moduleName ]

                Just _ ->
                    let
                        dependentModules =
                            repo.moduleDependencies |> DAG.incomingEdges moduleName
                    in
                    if Set.isEmpty dependentModules then
                        Nothing

                    else
                        Just [ ModuleHasDependents moduleName dependentModules ]
    in
    validationErrors
        |> Maybe.map Err
        |> Maybe.withDefault
            (Ok
                { repo
                    | modules =
                        repo.modules
                            |> Dict.remove moduleName
                    , moduleDependencies =
                        repo.moduleDependencies
                            |> DAG.removeNodeAndSubtrees moduleName
                }
            )


insertType : ModuleName -> Name -> Type.Definition () -> Repo -> Result Errors Repo
insertType moduleName typeName typeDef repo =
    case repo.modules |> Dict.get moduleName of
        Just modDefinition ->
            case modDefinition.value.types |> Dict.get typeName of
                Just _ ->
                    Err [ TypeAlreadyExist typeName ]

                Nothing ->
                    let
                        updateModule : Maybe (AccessControlled (Module.Definition () (Type ()))) -> Maybe (AccessControlled (Module.Definition () (Type ())))
                        updateModule maybeModuleDefinition =
                            maybeModuleDefinition
                                |> Maybe.map
                                    (\accessControlledModuleDef ->
                                        accessControlledModuleDef
                                            |> AccessControlled.map
                                                (\moduleDef ->
                                                    { moduleDef
                                                        | types =
                                                            modDefinition.value.types
                                                                |> Dict.insert typeName (public (typeDef |> Documented.Documented ""))
                                                    }
                                                )
                                    )
                    in
                    Ok
                        { repo
                            | modules =
                                repo.modules
                                    |> Dict.update moduleName updateModule
                        }

        Nothing ->
            Err [ ModuleNotFound moduleName ]


withAccessControl : Bool -> a -> AccessControlled a
withAccessControl isExposed value =
    if isExposed then
        AccessControlled.public value

    else
        AccessControlled.private value


getPackageName : Repo -> PackageName
getPackageName repo =
    repo.packageName


dependsOnPackages : Repo -> Set PackageName
dependsOnPackages repo =
    repo.dependencies
        |> Dict.keys
        |> Set.fromList


lookupModuleSpecification : PackageName -> ModuleName -> Repo -> Maybe (Module.Specification ())
lookupModuleSpecification packageName moduleName repo =
    if packageName == repo.packageName then
        repo.modules
            |> Dict.get moduleName
            -- Private modules are visible within the same package
            |> Maybe.map AccessControlled.withPrivateAccess
            |> Maybe.map Module.definitionToSpecification

    else
        repo.dependencies
            |> Dict.get packageName
            |> Maybe.andThen (.modules >> Dict.get moduleName)
            -- Private modules are not visible from another package
            |> Maybe.andThen AccessControlled.withPublicAccess
            |> Maybe.map Module.definitionToSpecification
