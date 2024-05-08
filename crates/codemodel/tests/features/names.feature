Feature: Names Feature

    Scenario: Creating a Name from a string
        Given a name of 'John Doe'
        When I create a Name from the string
        Then the segments should be: "john", "doe"
