## Language

- Allowing underscores in numbers (ie 1_000_000)
- Ranges
- String interpolation

## Theory Crafting

- Newline
  - Expressions end in newline but so do if conditionals
  - Could have a different token for ':\n'
  - Could just not send a newline token for ':\n' since it has indent which acts as an open brace.
  - Is that too much logic for the lexer to know?

## Tests

- [ ] Error_number_parse_failure
  - Have a number token with an incorrect lexeme ("2.a") and see what happens
