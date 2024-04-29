port module Main exposing (main)

import Platform exposing (Program)

type alias InputType = String
type alias OutputType = List String

port get : (InputType -> msg) -> Sub msg

port put : OutputType -> Cmd msg

main: Program Flags Model Msg 
main =
    Platform.worker
        { init = init
        , update = update
        , subscriptions = subscriptions
        }

type alias Flags = ()

type alias Model = List String

type Msg 
    = Input String


init:Flags -> (Model, Cmd Msg)
init _ = ([], Cmd.none)

update: Msg -> Model -> (Model, Cmd Msg)
update msg model = 
    case msg of 
        Input str -> 
            let
                newModel = model ++ [str]
            in            
                (newModel, newModel |> put)

subscriptions : Model -> Sub Msg
subscriptions _ =
    get Input
