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
        repeat($.statement),
      '}'
    ),

    statement: $ => seq(
      choice(
        $.assignment,
        $._expression,
        $.choice,
        $.while,
        $.function,
      ),
      optional(';')
    ),
    function: $ => seq(
      'fn',
      $.name,
      $.params,
      $.block
    ),

    while: $ => seq(
      'while',
      $._parenthesis,
      $.block
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
      prec.left(4, seq($._expression, '%', $._expression)),
      prec.left(3, seq($._expression, '*', $._expression)),
      prec.left(3, seq($._expression, '/', $._expression)),
      prec.left(2, seq($._expression, '-', $._expression)),
      prec.left(2, seq($._expression, '+', $._expression)),
      prec.left(1, seq($._expression, '==', $._expression)),
      prec.left(1, seq($._expression, '!=', $._expression)),
    ),

    _noun: $ => choice(
      $.identifier,
      $.number,
      $.string,
      $.call,
    ),

    call: $ => prec(1, seq(
      prec(3, $.name),
      $.arguments
    )),

    params: $ => seq(
      '(',
      optional(seq(
        $.param,
        repeat(seq(',', $.param))
      )),
      ')'
    ),

    arguments: $ => seq(
      '(',
      optional(seq(
        $._expression,
        repeat(seq(',', $._expression))
      )),
      ')'
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
    param: $ => /[a-zA-Z][a-zA-Z0-9]*/,
    name: $ => /[a-zA-Z][a-zA-Z0-9]*/,
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
