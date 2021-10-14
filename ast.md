# AST Structure

A tree is any of the following:
 - An integer
 - A decimal
 - TODO: support a number in scientific notation
 - A free variable (this can include greek letters/subscripts)
 - TBD list of non-variable symbols to leave (ellipses, infinity, DNE)
 - One of the following binary expressions:
   - Addition
   - Subtraction
   - Multiplication
   - Division
   - Exponentiation
   - Logarithms
   - Modulo
   - Any of the binary comparison relations ≤, ≥, <, >, =, ≠, ~
   - The improper and proper subset relations
   - Logical and, or, xor, and implication (this last one also serving as a general arrow between two equations)
 - One of the following unary expressions:
   - The unary minus and plus operator
   - The logical not operator
   - TODO: possible display control might go here (bold, strikethrough, italic, etc.)
 - A function with any amount of arguments
