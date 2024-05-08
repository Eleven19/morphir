Feature: Module Feature

    Scenario: Creating a Name from a string
        Given a string "John Doe"
        When I create a Name from the string
        Then the segments should be:
            | segments|
            | john |
            | doe |