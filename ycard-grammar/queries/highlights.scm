; Syntax highlighting for yCard

; Keywords and field names
(block_mapping_pair
  key: (plain_scalar) @property)

(flow_mapping_pair
  key: (plain_scalar) @property)

; Special shorthand keys
(phone_shorthand_key) @property.builtin

(email_shorthand_key) @property.builtin

; Values
(plain_scalar) @string
(double_quoted_scalar) @string
(single_quoted_scalar) @string

; Numbers
(plain_scalar) @number
  (#match? @number "^[0-9]+$")

; Phone numbers
(plain_scalar) @constant.numeric
  (#match? @constant.numeric "^\\+?[0-9\\s\\-\\(\\)\\.\\/ext]+$")

; Email addresses
(plain_scalar) @constant
  (#match? @constant "^[^@]+@[^@]+\\.[^@]+$")

; Booleans
(plain_scalar) @boolean
  (#match? @boolean "^(true|false|yes|no|on|off)$")

; Special yCard version
(block_mapping_pair
  key: (plain_scalar) @keyword
  (#eq? @keyword "version"))

; Comments
(comment) @comment

; Punctuation
":" @punctuation.delimiter
"," @punctuation.delimiter
"-" @punctuation.special

; Brackets
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

; Quotes
"\"" @punctuation.delimiter
"'" @punctuation.delimiter