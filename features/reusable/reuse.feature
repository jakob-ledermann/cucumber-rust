Feature: Reusable Test on Trait objects

  Scenario: Sorting
    Given A list of elements [1,5,2,4,3]
    When I Sort the list
    Then I should receive a list with [1,2,3,4,5]
