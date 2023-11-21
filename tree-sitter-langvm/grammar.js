module.exports = grammar({
  name: 'langvm',

  extras: $ => [
    $.comment,
    /\s/,
  ],

  rules: {
    source_file: $ => repeat($.statement),

    block: $ => seq(
      '{',
        repeat($._expression),
      '}'
    ),

    statement: $ => seq(
      choice(
        $.assignment,
        $._expression,
        $.choice
      ),
      ';'
    ),
    choice: $ => seq(
      'if',
      $._parenthesis,
      $.block,
      optional(seq('else', $.block))
    ),
    _expression: $ => choice(
      $.binary_expression,
      $._noun,
      $._parenthesis,
      $.block,
    ),

    binary_expression: $ => choice(
      prec.left(2, seq($._expression, '*', $._expression)),
      prec.left(2, seq($._expression, '/', $._expression)),
      prec.left(1, seq($._expression, '-', $._expression)),
      prec.left(1, seq($._expression, '+', $._expression)),
    ),

    _noun: $ => choice(
      $.identifier,
      $.number,
      $.string,
    ),

    _parenthesis: $ => seq(
      '(',
      $._expression,
      ')'
    ),
    assignment: $ => prec.left(0, seq($.identifier, "=", $._expression)),

    string: $ => choice(
      $._string_double,
      $._string_single,
    ),
    _string_double: $ => seq(
      '"',
      /[^"]*/,
      '"'
    ),
    _string_single: $ => seq(
      "'",
      /[^']*/,
      "'"
    ),
    identifier: $ => /[a-zA-Z][a-zA-Z0-9]*/,
    _number_dec: $ => /[1-9]\d*/,
    _number_oct: $ => /0[0-7]*/,
    _number_hex: $ => /0x[0-9a-f]+/,
    number: $ => choice(
      $._number_dec,
      $._number_oct,
      $._number_hex
    ),
    comment: $ => token(choice(
      seq('//', /.*/),
      seq(
        '/*',
        /[^*]*\*+([^/*][^*]*\*+)*/,
        '/',
      ),
    )),

  }
});
