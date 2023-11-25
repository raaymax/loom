"fn" @keyword
"while" @keyword
"return" @keyword
"if" @keyword
"else" @keyword
"==" @operator
"+" @operator
"-" @operator
"!=" @operator
"*" @operator
"/" @operator
"%" @operator
"=" @operator.assignment
(call (name) @method)
(function (name) @function.name
          (params (param) @parameter))
(assignment) @assignment
(number) @number
(string) @string
(comment) @comment
(identifier) @variable
