//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A packrat parser for the ZeroQL language.
///
/// It is essantially a recursive descent parser that memoizes the results of parsing subexpressions,
/// which allows it to parse any context-free grammar in linear time. In addition to that, the parser
/// also uses state backtracking to handle ambiguous grammars.
///
/// It is based on the grammar defined in the `parser.grammar` file.
pub struct Parser {
    // ...
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Parser {
    // ...
}
