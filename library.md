# Zyxtlang stdlib plans
* std
  * math
    * ifx rt(a: #Num, b: #Num): #Num
    * ifx log(a: #Num, b: #Num): #Num
    * log10
    * log2
    * ln
    * ext #Num.factorial(): #Num
    * ext #Num.gamma(): #Num
    * bit
      * ext #Num.compl(): #Num
      * ifx and
      * or
      * xor
      * lsh
      * rsh
      * zrsh
    * geo
      * trigo
        * sin
        * cos
        * tan
        * csc
        * sec
        * cot
      * coord
        * to_polar(coords: #varg<#Num>): tuple<#arb<#Num>>
        * to_cartesian(r: #Num, angles: #varg<#Num>): tuple<#arb<#Num>>
  * console
  * requests
  * sys
  * async?
  * collections
  * media?
  * datetime
  * argparse
* str