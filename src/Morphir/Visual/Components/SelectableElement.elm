module Morphir.Visual.Components.SelectableElement exposing (..)

import Element exposing (Element, el, fill, mouseOver, pointer, width)
import Element.Background as Background
import Element.Border as Border
import Element.Events exposing (onClick)
import Element.Font as Font
import Morphir.Visual.Theme exposing (Theme, lightMorphIrBlue)


type alias Config msg =
    { onSelect : msg
    , content : Element msg
    , isSelected : Bool
    }


view : Theme -> Config msg -> Element msg
view theme config =
    let
        styles =
            [ width fill
            , pointer
            , Border.widthEach { left = 2, right = 0, bottom = 0, top = 0 }
            , Border.color theme.colors.lightest
            , mouseOver [ Background.color lightMorphIrBlue, Border.color theme.colors.primaryHighlight ]
            ]

        activeStyles =
            if config.isSelected then
                [ Font.bold, Background.color lightMorphIrBlue, Border.color theme.colors.primaryHighlight ]

            else
                []
    in
    el ((onClick config.onSelect :: styles) ++ activeStyles) config.content
