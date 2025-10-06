const PREC = {
  comment: 1,
  anchor: 2,
  tag: 3,
  string: 4,
  number: 5,
};

// Import generated keys from schema
const { PHONE_SHORTHAND_KEYS, EMAIL_SHORTHAND_KEYS } = require('./generated_keys');

module.exports = grammar({
  name: 'ycard',

  extras: $ => [
    /\s/,
    $.comment,
  ],

  rules: {
    source_file: $ => repeat($._definition),

    _definition: $ => choice(
      $.block_mapping_pair,
      $.block_sequence_item,
      $.flow_mapping,
      $.flow_sequence,
      $.comment,
    ),

    // Block mappings (key: value)
    block_mapping_pair: $ => seq(
      field('key', choice(
        $.plain_scalar,
        $.phone_shorthand_key,
        $.email_shorthand_key,
        $.quoted_scalar,
      )),
      ':',
      optional($._block_value),
    ),

    phone_shorthand_key: $ => choice(...PHONE_SHORTHAND_KEYS),

    email_shorthand_key: $ => choice(...EMAIL_SHORTHAND_KEYS),

    _block_value: $ => choice(
      $.plain_scalar,
      $.quoted_scalar,
      $.block_mapping,
      $.block_sequence,
      $.flow_mapping,
      $.flow_sequence,
    ),

    // Block sequences
    block_sequence: $ => repeat1($.block_sequence_item),

    block_sequence_item: $ => seq(
      '-',
      optional($._block_value),
    ),

    // Flow styles
    flow_mapping: $ => seq(
      '{',
      optional($._flow_mapping_content),
      '}',
    ),

    _flow_mapping_content: $ => seq(
      $.flow_mapping_pair,
      repeat(seq(',', $.flow_mapping_pair)),
      optional(','),
    ),

    flow_mapping_pair: $ => seq(
      field('key', choice($.plain_scalar, $.quoted_scalar)),
      ':',
      field('value', choice($.plain_scalar, $.quoted_scalar, $.flow_mapping, $.flow_sequence)),
    ),

    flow_sequence: $ => seq(
      '[',
      optional($._flow_sequence_content),
      ']',
    ),

    _flow_sequence_content: $ => seq(
      choice($.plain_scalar, $.quoted_scalar, $.flow_mapping, $.flow_sequence),
      repeat(seq(',', choice($.plain_scalar, $.quoted_scalar, $.flow_mapping, $.flow_sequence))),
      optional(','),
    ),

    // Block mapping
    block_mapping: $ => repeat1($.block_mapping_pair),

    // Scalars
    plain_scalar: $ => token(prec(PREC.string, /[^\s\[\]{},:]+/)),

    quoted_scalar: $ => choice(
      $.double_quoted_scalar,
      $.single_quoted_scalar,
    ),

    double_quoted_scalar: $ => seq(
      '"',
      repeat(choice(
        token.immediate(prec(1, /[^"\\]+/)),
        $.escape_sequence,
      )),
      '"',
    ),

    single_quoted_scalar: $ => seq(
      "'",
      repeat(choice(
        token.immediate(prec(1, /[^']+/)),
        "''", // Escaped single quote
      )),
      "'",
    ),

    escape_sequence: $ => token.immediate(seq(
      '\\',
      choice(
        /["\\/bfnrt]/,
        /u[0-9a-fA-F]{4}/,
        /U[0-9a-fA-F]{8}/,
      ),
    )),

    // Comments
    comment: $ => token(prec(PREC.comment, /#.*/)),
  }
});