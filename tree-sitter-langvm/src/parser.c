#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 41
#define LARGE_STATE_COUNT 9
#define SYMBOL_COUNT 37
#define ALIAS_COUNT 0
#define TOKEN_COUNT 22
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 1

enum {
  anon_sym_LBRACE = 1,
  anon_sym_RBRACE = 2,
  anon_sym_SEMI = 3,
  anon_sym_if = 4,
  anon_sym_else = 5,
  anon_sym_STAR = 6,
  anon_sym_SLASH = 7,
  anon_sym_DASH = 8,
  anon_sym_PLUS = 9,
  anon_sym_LPAREN = 10,
  anon_sym_RPAREN = 11,
  anon_sym_EQ = 12,
  anon_sym_DQUOTE = 13,
  aux_sym__string_double_token1 = 14,
  anon_sym_SQUOTE = 15,
  aux_sym__string_single_token1 = 16,
  sym_identifier = 17,
  sym__number_dec = 18,
  sym__number_oct = 19,
  sym__number_hex = 20,
  sym_comment = 21,
  sym_source_file = 22,
  sym_block = 23,
  sym_statement = 24,
  sym_choice = 25,
  sym__expression = 26,
  sym_binary_expression = 27,
  sym__noun = 28,
  sym__parenthesis = 29,
  sym_assignment = 30,
  sym_string = 31,
  sym__string_double = 32,
  sym__string_single = 33,
  sym_number = 34,
  aux_sym_source_file_repeat1 = 35,
  aux_sym_block_repeat1 = 36,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_SEMI] = ";",
  [anon_sym_if] = "if",
  [anon_sym_else] = "else",
  [anon_sym_STAR] = "*",
  [anon_sym_SLASH] = "/",
  [anon_sym_DASH] = "-",
  [anon_sym_PLUS] = "+",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_EQ] = "=",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym__string_double_token1] = "_string_double_token1",
  [anon_sym_SQUOTE] = "'",
  [aux_sym__string_single_token1] = "_string_single_token1",
  [sym_identifier] = "identifier",
  [sym__number_dec] = "_number_dec",
  [sym__number_oct] = "_number_oct",
  [sym__number_hex] = "_number_hex",
  [sym_comment] = "comment",
  [sym_source_file] = "source_file",
  [sym_block] = "block",
  [sym_statement] = "statement",
  [sym_choice] = "choice",
  [sym__expression] = "_expression",
  [sym_binary_expression] = "binary_expression",
  [sym__noun] = "_noun",
  [sym__parenthesis] = "_parenthesis",
  [sym_assignment] = "assignment",
  [sym_string] = "string",
  [sym__string_double] = "_string_double",
  [sym__string_single] = "_string_single",
  [sym_number] = "number",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_else] = anon_sym_else,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym__string_double_token1] = aux_sym__string_double_token1,
  [anon_sym_SQUOTE] = anon_sym_SQUOTE,
  [aux_sym__string_single_token1] = aux_sym__string_single_token1,
  [sym_identifier] = sym_identifier,
  [sym__number_dec] = sym__number_dec,
  [sym__number_oct] = sym__number_oct,
  [sym__number_hex] = sym__number_hex,
  [sym_comment] = sym_comment,
  [sym_source_file] = sym_source_file,
  [sym_block] = sym_block,
  [sym_statement] = sym_statement,
  [sym_choice] = sym_choice,
  [sym__expression] = sym__expression,
  [sym_binary_expression] = sym_binary_expression,
  [sym__noun] = sym__noun,
  [sym__parenthesis] = sym__parenthesis,
  [sym_assignment] = sym_assignment,
  [sym_string] = sym_string,
  [sym__string_double] = sym__string_double,
  [sym__string_single] = sym__string_single,
  [sym_number] = sym_number,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_block_repeat1] = aux_sym_block_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_if] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_else] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym__string_double_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_SQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym__string_single_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym__number_dec] = {
    .visible = false,
    .named = true,
  },
  [sym__number_oct] = {
    .visible = false,
    .named = true,
  },
  [sym__number_hex] = {
    .visible = false,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_choice] = {
    .visible = true,
    .named = true,
  },
  [sym__expression] = {
    .visible = false,
    .named = true,
  },
  [sym_binary_expression] = {
    .visible = true,
    .named = true,
  },
  [sym__noun] = {
    .visible = false,
    .named = true,
  },
  [sym__parenthesis] = {
    .visible = false,
    .named = true,
  },
  [sym_assignment] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym__string_double] = {
    .visible = false,
    .named = true,
  },
  [sym__string_single] = {
    .visible = false,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_block_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 4,
  [8] = 6,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 21,
  [31] = 31,
  [32] = 16,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(11);
      if (lookahead == '"') ADVANCE(25);
      if (lookahead == '\'') ADVANCE(32);
      if (lookahead == '(') ADVANCE(22);
      if (lookahead == ')') ADVANCE(23);
      if (lookahead == '*') ADVANCE(18);
      if (lookahead == '+') ADVANCE(21);
      if (lookahead == '-') ADVANCE(20);
      if (lookahead == '/') ADVANCE(19);
      if (lookahead == '0') ADVANCE(45);
      if (lookahead == ';') ADVANCE(14);
      if (lookahead == '=') ADVANCE(24);
      if (lookahead == 'e') ADVANCE(41);
      if (lookahead == 'i') ADVANCE(40);
      if (lookahead == '{') ADVANCE(12);
      if (lookahead == '}') ADVANCE(13);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(44);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(25);
      if (lookahead == '\'') ADVANCE(32);
      if (lookahead == '(') ADVANCE(22);
      if (lookahead == ')') ADVANCE(23);
      if (lookahead == '*') ADVANCE(18);
      if (lookahead == '+') ADVANCE(21);
      if (lookahead == '-') ADVANCE(20);
      if (lookahead == '/') ADVANCE(19);
      if (lookahead == '0') ADVANCE(45);
      if (lookahead == ';') ADVANCE(14);
      if (lookahead == '{') ADVANCE(12);
      if (lookahead == '}') ADVANCE(13);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(44);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      END_STATE();
    case 2:
      if (lookahead == '*') ADVANCE(4);
      if (lookahead == '/') ADVANCE(49);
      END_STATE();
    case 3:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead == '/') ADVANCE(48);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 4:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 5:
      if (lookahead == '/') ADVANCE(2);
      if (lookahead == ';') ADVANCE(14);
      if (lookahead == 'e') ADVANCE(7);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(5)
      END_STATE();
    case 6:
      if (lookahead == 'e') ADVANCE(16);
      END_STATE();
    case 7:
      if (lookahead == 'l') ADVANCE(8);
      END_STATE();
    case 8:
      if (lookahead == 's') ADVANCE(6);
      END_STATE();
    case 9:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(47);
      END_STATE();
    case 10:
      if (eof) ADVANCE(11);
      if (lookahead == '"') ADVANCE(25);
      if (lookahead == '\'') ADVANCE(32);
      if (lookahead == '(') ADVANCE(22);
      if (lookahead == '/') ADVANCE(2);
      if (lookahead == '0') ADVANCE(45);
      if (lookahead == 'i') ADVANCE(40);
      if (lookahead == '{') ADVANCE(12);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(10)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(44);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_if);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_else);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(4);
      if (lookahead == '/') ADVANCE(49);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(aux_sym__string_double_token1);
      if (lookahead == '\n') ADVANCE(31);
      if (lookahead == '"') ADVANCE(49);
      if (lookahead != 0) ADVANCE(26);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(aux_sym__string_double_token1);
      if (lookahead == '"') ADVANCE(4);
      if (lookahead == '*') ADVANCE(27);
      if (lookahead == '/') ADVANCE(31);
      if (lookahead != 0) ADVANCE(28);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(aux_sym__string_double_token1);
      if (lookahead == '"') ADVANCE(4);
      if (lookahead == '*') ADVANCE(27);
      if (lookahead != 0) ADVANCE(28);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(aux_sym__string_double_token1);
      if (lookahead == '*') ADVANCE(28);
      if (lookahead == '/') ADVANCE(26);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(31);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(aux_sym__string_double_token1);
      if (lookahead == '/') ADVANCE(29);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(30);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(31);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(aux_sym__string_double_token1);
      if (lookahead != 0 &&
          lookahead != '"') ADVANCE(31);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_SQUOTE);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(aux_sym__string_single_token1);
      if (lookahead == '\n') ADVANCE(38);
      if (lookahead == '\'') ADVANCE(49);
      if (lookahead != 0) ADVANCE(33);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(aux_sym__string_single_token1);
      if (lookahead == '\'') ADVANCE(4);
      if (lookahead == '*') ADVANCE(34);
      if (lookahead == '/') ADVANCE(38);
      if (lookahead != 0) ADVANCE(35);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(aux_sym__string_single_token1);
      if (lookahead == '\'') ADVANCE(4);
      if (lookahead == '*') ADVANCE(34);
      if (lookahead != 0) ADVANCE(35);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(aux_sym__string_single_token1);
      if (lookahead == '*') ADVANCE(35);
      if (lookahead == '/') ADVANCE(33);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(38);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(aux_sym__string_single_token1);
      if (lookahead == '/') ADVANCE(36);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(37);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(38);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(aux_sym__string_single_token1);
      if (lookahead != 0 &&
          lookahead != '\'') ADVANCE(38);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(17);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'f') ADVANCE(15);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(42);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(39);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym__number_dec);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(44);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym__number_oct);
      if (lookahead == 'x') ADVANCE(9);
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(46);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym__number_oct);
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(46);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(sym__number_hex);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(47);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(49);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 10},
  [2] = {.lex_state = 10},
  [3] = {.lex_state = 10},
  [4] = {.lex_state = 1},
  [5] = {.lex_state = 1},
  [6] = {.lex_state = 1},
  [7] = {.lex_state = 1},
  [8] = {.lex_state = 1},
  [9] = {.lex_state = 1},
  [10] = {.lex_state = 1},
  [11] = {.lex_state = 1},
  [12] = {.lex_state = 1},
  [13] = {.lex_state = 1},
  [14] = {.lex_state = 1},
  [15] = {.lex_state = 1},
  [16] = {.lex_state = 1},
  [17] = {.lex_state = 1},
  [18] = {.lex_state = 1},
  [19] = {.lex_state = 1},
  [20] = {.lex_state = 1},
  [21] = {.lex_state = 1},
  [22] = {.lex_state = 1},
  [23] = {.lex_state = 10},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 5},
  [30] = {.lex_state = 5},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 5},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 30},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 37},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [anon_sym_SQUOTE] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [sym__number_dec] = ACTIONS(1),
    [sym__number_oct] = ACTIONS(1),
    [sym__number_hex] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(39),
    [sym_block] = STATE(25),
    [sym_statement] = STATE(2),
    [sym_choice] = STATE(34),
    [sym__expression] = STATE(25),
    [sym_binary_expression] = STATE(25),
    [sym__noun] = STATE(25),
    [sym__parenthesis] = STATE(25),
    [sym_assignment] = STATE(34),
    [sym_string] = STATE(25),
    [sym__string_double] = STATE(15),
    [sym__string_single] = STATE(15),
    [sym_number] = STATE(25),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_DQUOTE] = ACTIONS(13),
    [anon_sym_SQUOTE] = ACTIONS(15),
    [sym_identifier] = ACTIONS(17),
    [sym__number_dec] = ACTIONS(19),
    [sym__number_oct] = ACTIONS(21),
    [sym__number_hex] = ACTIONS(19),
    [sym_comment] = ACTIONS(3),
  },
  [2] = {
    [sym_block] = STATE(25),
    [sym_statement] = STATE(3),
    [sym_choice] = STATE(34),
    [sym__expression] = STATE(25),
    [sym_binary_expression] = STATE(25),
    [sym__noun] = STATE(25),
    [sym__parenthesis] = STATE(25),
    [sym_assignment] = STATE(34),
    [sym_string] = STATE(25),
    [sym__string_double] = STATE(15),
    [sym__string_single] = STATE(15),
    [sym_number] = STATE(25),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(23),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_DQUOTE] = ACTIONS(13),
    [anon_sym_SQUOTE] = ACTIONS(15),
    [sym_identifier] = ACTIONS(17),
    [sym__number_dec] = ACTIONS(19),
    [sym__number_oct] = ACTIONS(21),
    [sym__number_hex] = ACTIONS(19),
    [sym_comment] = ACTIONS(3),
  },
  [3] = {
    [sym_block] = STATE(25),
    [sym_statement] = STATE(3),
    [sym_choice] = STATE(34),
    [sym__expression] = STATE(25),
    [sym_binary_expression] = STATE(25),
    [sym__noun] = STATE(25),
    [sym__parenthesis] = STATE(25),
    [sym_assignment] = STATE(34),
    [sym_string] = STATE(25),
    [sym__string_double] = STATE(15),
    [sym__string_single] = STATE(15),
    [sym_number] = STATE(25),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(25),
    [anon_sym_LBRACE] = ACTIONS(27),
    [anon_sym_if] = ACTIONS(30),
    [anon_sym_LPAREN] = ACTIONS(33),
    [anon_sym_DQUOTE] = ACTIONS(36),
    [anon_sym_SQUOTE] = ACTIONS(39),
    [sym_identifier] = ACTIONS(42),
    [sym__number_dec] = ACTIONS(45),
    [sym__number_oct] = ACTIONS(48),
    [sym__number_hex] = ACTIONS(45),
    [sym_comment] = ACTIONS(3),
  },
  [4] = {
    [sym_block] = STATE(22),
    [sym__expression] = STATE(22),
    [sym_binary_expression] = STATE(22),
    [sym__noun] = STATE(22),
    [sym__parenthesis] = STATE(22),
    [sym_string] = STATE(22),
    [sym__string_double] = STATE(15),
    [sym__string_single] = STATE(15),
    [sym_number] = STATE(22),
    [aux_sym_block_repeat1] = STATE(8),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_RBRACE] = ACTIONS(51),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_DQUOTE] = ACTIONS(13),
    [anon_sym_SQUOTE] = ACTIONS(15),
    [sym_identifier] = ACTIONS(53),
    [sym__number_dec] = ACTIONS(19),
    [sym__number_oct] = ACTIONS(21),
    [sym__number_hex] = ACTIONS(19),
    [sym_comment] = ACTIONS(3),
  },
  [5] = {
    [sym_block] = STATE(22),
    [sym__expression] = STATE(22),
    [sym_binary_expression] = STATE(22),
    [sym__noun] = STATE(22),
    [sym__parenthesis] = STATE(22),
    [sym_string] = STATE(22),
    [sym__string_double] = STATE(15),
    [sym__string_single] = STATE(15),
    [sym_number] = STATE(22),
    [aux_sym_block_repeat1] = STATE(5),
    [anon_sym_LBRACE] = ACTIONS(55),
    [anon_sym_RBRACE] = ACTIONS(58),
    [anon_sym_LPAREN] = ACTIONS(60),
    [anon_sym_DQUOTE] = ACTIONS(63),
    [anon_sym_SQUOTE] = ACTIONS(66),
    [sym_identifier] = ACTIONS(69),
    [sym__number_dec] = ACTIONS(72),
    [sym__number_oct] = ACTIONS(75),
    [sym__number_hex] = ACTIONS(72),
    [sym_comment] = ACTIONS(3),
  },
  [6] = {
    [sym_block] = STATE(22),
    [sym__expression] = STATE(22),
    [sym_binary_expression] = STATE(22),
    [sym__noun] = STATE(22),
    [sym__parenthesis] = STATE(22),
    [sym_string] = STATE(22),
    [sym__string_double] = STATE(15),
    [sym__string_single] = STATE(15),
    [sym_number] = STATE(22),
    [aux_sym_block_repeat1] = STATE(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_RBRACE] = ACTIONS(78),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_DQUOTE] = ACTIONS(13),
    [anon_sym_SQUOTE] = ACTIONS(15),
    [sym_identifier] = ACTIONS(53),
    [sym__number_dec] = ACTIONS(19),
    [sym__number_oct] = ACTIONS(21),
    [sym__number_hex] = ACTIONS(19),
    [sym_comment] = ACTIONS(3),
  },
  [7] = {
    [sym_block] = STATE(22),
    [sym__expression] = STATE(22),
    [sym_binary_expression] = STATE(22),
    [sym__noun] = STATE(22),
    [sym__parenthesis] = STATE(22),
    [sym_string] = STATE(22),
    [sym__string_double] = STATE(15),
    [sym__string_single] = STATE(15),
    [sym_number] = STATE(22),
    [aux_sym_block_repeat1] = STATE(6),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_RBRACE] = ACTIONS(80),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_DQUOTE] = ACTIONS(13),
    [anon_sym_SQUOTE] = ACTIONS(15),
    [sym_identifier] = ACTIONS(53),
    [sym__number_dec] = ACTIONS(19),
    [sym__number_oct] = ACTIONS(21),
    [sym__number_hex] = ACTIONS(19),
    [sym_comment] = ACTIONS(3),
  },
  [8] = {
    [sym_block] = STATE(22),
    [sym__expression] = STATE(22),
    [sym_binary_expression] = STATE(22),
    [sym__noun] = STATE(22),
    [sym__parenthesis] = STATE(22),
    [sym_string] = STATE(22),
    [sym__string_double] = STATE(15),
    [sym__string_single] = STATE(15),
    [sym_number] = STATE(22),
    [aux_sym_block_repeat1] = STATE(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_RBRACE] = ACTIONS(82),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_DQUOTE] = ACTIONS(13),
    [anon_sym_SQUOTE] = ACTIONS(15),
    [sym_identifier] = ACTIONS(53),
    [sym__number_dec] = ACTIONS(19),
    [sym__number_oct] = ACTIONS(21),
    [sym__number_hex] = ACTIONS(19),
    [sym_comment] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_LBRACE,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(13), 1,
      anon_sym_DQUOTE,
    ACTIONS(15), 1,
      anon_sym_SQUOTE,
    ACTIONS(21), 1,
      sym__number_oct,
    ACTIONS(84), 1,
      sym_identifier,
    ACTIONS(19), 2,
      sym__number_dec,
      sym__number_hex,
    STATE(15), 2,
      sym__string_double,
      sym__string_single,
    STATE(20), 7,
      sym_block,
      sym__expression,
      sym_binary_expression,
      sym__noun,
      sym__parenthesis,
      sym_string,
      sym_number,
  [39] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_LBRACE,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(13), 1,
      anon_sym_DQUOTE,
    ACTIONS(15), 1,
      anon_sym_SQUOTE,
    ACTIONS(21), 1,
      sym__number_oct,
    ACTIONS(86), 1,
      sym_identifier,
    ACTIONS(19), 2,
      sym__number_dec,
      sym__number_hex,
    STATE(15), 2,
      sym__string_double,
      sym__string_single,
    STATE(26), 7,
      sym_block,
      sym__expression,
      sym_binary_expression,
      sym__noun,
      sym__parenthesis,
      sym_string,
      sym_number,
  [78] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_LBRACE,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(13), 1,
      anon_sym_DQUOTE,
    ACTIONS(15), 1,
      anon_sym_SQUOTE,
    ACTIONS(21), 1,
      sym__number_oct,
    ACTIONS(88), 1,
      sym_identifier,
    ACTIONS(19), 2,
      sym__number_dec,
      sym__number_hex,
    STATE(15), 2,
      sym__string_double,
      sym__string_single,
    STATE(14), 7,
      sym_block,
      sym__expression,
      sym_binary_expression,
      sym__noun,
      sym__parenthesis,
      sym_string,
      sym_number,
  [117] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_LBRACE,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    ACTIONS(13), 1,
      anon_sym_DQUOTE,
    ACTIONS(15), 1,
      anon_sym_SQUOTE,
    ACTIONS(21), 1,
      sym__number_oct,
    ACTIONS(90), 1,
      sym_identifier,
    ACTIONS(19), 2,
      sym__number_dec,
      sym__number_hex,
    STATE(15), 2,
      sym__string_double,
      sym__string_single,
    STATE(27), 7,
      sym_block,
      sym__expression,
      sym_binary_expression,
      sym__noun,
      sym__parenthesis,
      sym_string,
      sym_number,
  [156] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(94), 2,
      anon_sym_SLASH,
      sym__number_oct,
    ACTIONS(92), 13,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
      sym_identifier,
      sym__number_dec,
      sym__number_hex,
  [179] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(98), 2,
      anon_sym_SLASH,
      sym__number_oct,
    ACTIONS(96), 13,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
      sym_identifier,
      sym__number_dec,
      sym__number_hex,
  [202] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(102), 2,
      anon_sym_SLASH,
      sym__number_oct,
    ACTIONS(100), 13,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
      sym_identifier,
      sym__number_dec,
      sym__number_hex,
  [225] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(106), 2,
      anon_sym_SLASH,
      sym__number_oct,
    ACTIONS(104), 13,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
      sym_identifier,
      sym__number_dec,
      sym__number_hex,
  [248] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(110), 2,
      anon_sym_SLASH,
      sym__number_oct,
    ACTIONS(108), 13,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
      sym_identifier,
      sym__number_dec,
      sym__number_hex,
  [271] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(114), 2,
      anon_sym_SLASH,
      sym__number_oct,
    ACTIONS(112), 13,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
      sym_identifier,
      sym__number_dec,
      sym__number_hex,
  [294] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(118), 2,
      anon_sym_SLASH,
      sym__number_oct,
    ACTIONS(116), 13,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
      sym_identifier,
      sym__number_dec,
      sym__number_hex,
  [317] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(98), 1,
      sym__number_oct,
    ACTIONS(120), 1,
      anon_sym_STAR,
    ACTIONS(122), 1,
      anon_sym_SLASH,
    ACTIONS(96), 12,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_SEMI,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
      sym_identifier,
      sym__number_dec,
      sym__number_hex,
  [344] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(126), 2,
      anon_sym_SLASH,
      sym__number_oct,
    ACTIONS(124), 13,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
      sym_identifier,
      sym__number_dec,
      sym__number_hex,
  [367] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(120), 1,
      anon_sym_STAR,
    ACTIONS(122), 1,
      anon_sym_SLASH,
    ACTIONS(132), 1,
      sym__number_oct,
    ACTIONS(130), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(128), 8,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_LPAREN,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
      sym_identifier,
      sym__number_dec,
      sym__number_hex,
  [394] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(136), 3,
      anon_sym_if,
      sym_identifier,
      sym__number_oct,
    ACTIONS(134), 7,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_LPAREN,
      anon_sym_DQUOTE,
      anon_sym_SQUOTE,
      sym__number_dec,
      sym__number_hex,
  [412] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(140), 1,
      anon_sym_SLASH,
    ACTIONS(142), 1,
      anon_sym_EQ,
    ACTIONS(138), 4,
      anon_sym_SEMI,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_PLUS,
  [428] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(120), 1,
      anon_sym_STAR,
    ACTIONS(122), 1,
      anon_sym_SLASH,
    ACTIONS(144), 1,
      anon_sym_SEMI,
    ACTIONS(130), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
  [445] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(120), 1,
      anon_sym_STAR,
    ACTIONS(122), 1,
      anon_sym_SLASH,
    ACTIONS(146), 1,
      anon_sym_SEMI,
    ACTIONS(130), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
  [462] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(120), 1,
      anon_sym_STAR,
    ACTIONS(122), 1,
      anon_sym_SLASH,
    ACTIONS(148), 1,
      anon_sym_RPAREN,
    ACTIONS(130), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
  [479] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(11), 1,
      anon_sym_LPAREN,
    STATE(33), 1,
      sym__parenthesis,
  [489] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(150), 1,
      anon_sym_SEMI,
    ACTIONS(152), 1,
      anon_sym_else,
  [499] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(124), 2,
      anon_sym_SEMI,
      anon_sym_else,
  [507] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      anon_sym_LBRACE,
    STATE(36), 1,
      sym_block,
  [517] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(104), 2,
      anon_sym_SEMI,
      anon_sym_else,
  [525] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(154), 1,
      anon_sym_LBRACE,
    STATE(29), 1,
      sym_block,
  [535] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(144), 1,
      anon_sym_SEMI,
  [542] = 2,
    ACTIONS(156), 1,
      aux_sym__string_double_token1,
    ACTIONS(158), 1,
      sym_comment,
  [549] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(160), 1,
      anon_sym_SEMI,
  [556] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(162), 1,
      anon_sym_SQUOTE,
  [563] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(164), 1,
      anon_sym_DQUOTE,
  [570] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(166), 1,
      ts_builtin_sym_end,
  [577] = 2,
    ACTIONS(158), 1,
      sym_comment,
    ACTIONS(168), 1,
      aux_sym__string_single_token1,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(9)] = 0,
  [SMALL_STATE(10)] = 39,
  [SMALL_STATE(11)] = 78,
  [SMALL_STATE(12)] = 117,
  [SMALL_STATE(13)] = 156,
  [SMALL_STATE(14)] = 179,
  [SMALL_STATE(15)] = 202,
  [SMALL_STATE(16)] = 225,
  [SMALL_STATE(17)] = 248,
  [SMALL_STATE(18)] = 271,
  [SMALL_STATE(19)] = 294,
  [SMALL_STATE(20)] = 317,
  [SMALL_STATE(21)] = 344,
  [SMALL_STATE(22)] = 367,
  [SMALL_STATE(23)] = 394,
  [SMALL_STATE(24)] = 412,
  [SMALL_STATE(25)] = 428,
  [SMALL_STATE(26)] = 445,
  [SMALL_STATE(27)] = 462,
  [SMALL_STATE(28)] = 479,
  [SMALL_STATE(29)] = 489,
  [SMALL_STATE(30)] = 499,
  [SMALL_STATE(31)] = 507,
  [SMALL_STATE(32)] = 517,
  [SMALL_STATE(33)] = 525,
  [SMALL_STATE(34)] = 535,
  [SMALL_STATE(35)] = 542,
  [SMALL_STATE(36)] = 549,
  [SMALL_STATE(37)] = 556,
  [SMALL_STATE(38)] = 563,
  [SMALL_STATE(39)] = 570,
  [SMALL_STATE(40)] = 577,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [27] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(4),
  [30] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(28),
  [33] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(12),
  [36] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(35),
  [39] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(40),
  [42] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(24),
  [45] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(17),
  [48] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(17),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [55] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(4),
  [58] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2),
  [60] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(12),
  [63] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(35),
  [66] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(40),
  [69] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(22),
  [72] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(17),
  [75] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2), SHIFT_REPEAT(17),
  [78] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [80] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [82] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [84] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [86] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [88] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [90] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__parenthesis, 3),
  [94] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__parenthesis, 3),
  [96] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_expression, 3),
  [98] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_expression, 3),
  [100] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 1),
  [102] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 1),
  [104] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [106] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2),
  [108] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 1),
  [110] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_number, 1),
  [112] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__string_single, 3),
  [114] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__string_single, 3),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__string_double, 3),
  [118] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__string_double, 3),
  [120] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [122] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [124] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [126] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3),
  [128] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 1),
  [130] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [132] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 1),
  [134] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 2),
  [136] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_statement, 2),
  [138] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__noun, 1),
  [140] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__noun, 1),
  [142] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [144] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [146] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assignment, 3),
  [148] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [150] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice, 3),
  [152] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [154] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [156] = {.entry = {.count = 1, .reusable = false}}, SHIFT(38),
  [158] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [160] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_choice, 5),
  [162] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [164] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [166] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [168] = {.entry = {.count = 1, .reusable = false}}, SHIFT(37),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_langvm(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
