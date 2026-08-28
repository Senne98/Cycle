### Variable parsing

Variables follow one of two structures: 
  1) single-character variable
  2) LaTeX variable

#### Single Character Variable

The variable consists of a single character. This should be a lower/upper case letter.

#### LaTeX Variable

The variable obeys the following structure:

  `\name^{upper}_{lower}`
  
The order of upper and lower indices can be switched. Unlike LaTeX, the curly braces can not be left out in any situation. The allowed characters differ for each position:
  * name: lower/upper case letter
  * upper: lower/upper case letter or numbers
  * lower: lower/upper case letter or numbers
