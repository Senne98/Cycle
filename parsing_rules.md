### Variable parsing

Variable follow one of two structures: 
  1) single character variable
  2) latex variable

#### Single Character Variable

The variable consists of a singular character. This should be a lower/upper case letter.

#### Latex Variable

The variable obeys following structure:
  `\name^{upper}_{lower}`
The order of upper and lower indices can be switched. Unlike latex, the curly braces can not be left out in any situation. The allowed characters differ for each position:
  * name: lower/upper case letter
  * upper: lower/upper case letter or numbers
  * lower: lower/upper case letter or numbers
