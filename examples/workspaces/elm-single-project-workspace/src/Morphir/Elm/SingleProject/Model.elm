module Morphir.Elm.SingleProject.Model exposing (..)

import Morphir.Elm.SingleProject.Model.AuthorName as AuthorName

type alias BookTitle = String
type alias Genre = String

type alias Book =
    { title : BookTitle
    , author : AuthorName.AuthorName
    , genre : String
    }