# Welcome to Jester Script

## Quick Q&A

### What is This Exactly?
Jester Script is a Lisp-inspired scripting langauge written in Rust

### Why Was This Created?

Because I wanted to. The name "Jester" was concieved because this project was originally supposed to be a joke - yet here we are. I went through countless iterations before reaching the present structure, and who knows what's to come in the future

### How is it Maintained?

Jester Script is solely maintained by me, so while it works great in testing, there's are no guarantees! If you're looking for a well-maintained scripting langauges, this isn't it 

## How Does it all Work?
```(+ 1 2 3)```; congrats, you just added ```1 + 2 + 3```!

Lisp and Jester Script use a variant on a prefix notation known as __S-Expressions__. These expressions follow the following form:
```
; (operator arguments)
(+ 1 2) ; operator = '+', arguments = '1, 2'
(* 3 4 5 6) ; operator = '*', arguments = '3, 4, 5, 6'
(println "hello there!") ; operator = 'println', arguments = '"hello there!"'
```

_In Lisp, `;` designates a comment - as if to ridicule all other langauges (but not really). 
Also note that everything is separated by whitespace... the comma will come into play later, but for a different cause_

Additionally, there is no "order of operations." Order is decided by the placement of the parentheses:

```
(2 + 3) * 5 => (* 5 (+ 2 3))
```

Operations can be nested as much as you want or need...

```
(+ 1 (+ 2 (+ 3 (+ 4 (+ 5))))) = (+ 1 2 3 4 5)
```

But there is more to S-Expressions, and this is where Jester Script diverges a bit from Lisp (if you want to read more on Lisp itself, there are great online resources). Consider the following:

```
; (argument.. arguments?)
(1 2 3)
(x y z)
("how" "are" "you?")
```

Luckliy enough, Jester Script doesn't completely break when this happens. In fact, this pattern is fundamental to its design. 

_S-Expressions without Operators are Lists!_

You'll come to see the true power of S-Expressions later on

__But Now What?__
It's time to _vary_ things up... with variables that is

In Jester Script, all variables exist from the get-go with a value of ```nil``` (equivalent to null, or None). Now, I obviously  don't store a list of all possible symbols--I don't have nearly enough patience for that. What does happen is that every time Jester Script parses out a symbol it hasn't encountered before (unless it is a numeric or String literal), it will add that symbol to the environment with a default `ni` value:

```
(println x) ; prints out 'nil'
(set x 10)
(println x) ; prints '10'
(set x (1 2 3))
(println x) ; prints '(1 2 3)'
```

_All variables are dynamically typed and mutable_

In Jester Script, everything is passed by value (with one caveat I'll later explain), meaning that, technically, there are no 'references'

```
(set x (1 2 3))
(set y x)
(append 4 x)
(println x) ; prints '(1 2 3 4)'
(println y) ; prints '(1 2 3)'
```

__Functions!__

Functions are defined in the same syntax as everything else:

```
(defun add (a b)
	(+ a b))

(println (add 10 20)) ; prints '30'
```

You might notice that the function has no `return`. Instead, Jester Script employs what is called a `progn`, which means that--for any expression--the last thing that evaluates also returns

Now lets explore the parallels between `defun` and `let`

Consider the following:

```
(let (a 10 b 20)
	(+ a b))
```

This is a `let` statement. It takes and sets parameters to a given argument `(10 -> a, 20 -> b)`, computes its body `(+ a b)` and returns the last expression. 

After exiting `let`, both `a` and `b` revert back to their previous values (in this case, lets assume they were `nil` prior). This is called a _Lexical Scope_, where variables maintain an "alias"

`let` and `defun` are identical save for the way they are invoked. In both cases we added `a` and `b` within a lexical scope, but `let` could only be invoked its declaration site with static arguments, while `defun` defines a variable that can be invoked with dynamic arguments. 

These similarities aren't a coicidence. Lisp and Jester follow a homogeneous design: _everything_ is and S-Expression, meaning that similarities between constructs arise where they were abstracted away before. 

__Back to Progns__

Remember, with S-Expressions, the last thing computed is also returned:

```
(+ 1 2 3) ; returns '6'
"hello" ; returns "hello"
(progn 1 2 3) ; returns '3,' the last thing the progn evaluated
``` 

__Now to REPL?__

Progns enable a unique feature of S-Expression languages: the Read-Eval-Print-Loop, aka REPL

A REPL works under the principle of 'last thing evaluated is returned'

```
; Jester Script REPL
>> x ; input
nil  ; output
>> (set x 10)
10 ; set still returns a value
>> (incr x 10)
20 ; incr also returns a value
>> x
20
>> (println "yay!")
yay!
"yay!" 
; what happened here?
; since println also returns whatever it prints, it 
; both logs it to the console then returns it again
; for the REPL to print for the second time
```
## Symbols

This section is so important it recieved its own big title!

Remember when I said (in air-quotes) that Jester has no references, well, it does have Quotes!

Lets look at an example:

```
>> (set x "my favorite variable")
"my favorite variable"
>> (set y (quote x))
X ; why is there a capital x here?
>> (eval y)
"my favorite variable"
>> (set x "less favorite variable")
"less favorite variable"
>> (eval y)
"less favorite variable"
```

_Note, because `quote` is so integral to Lisp, it has an integrated abbreviation (one of the few syntax abberations): `(quote x) = 'x`_

If you saw this and thought that `quote` is just a special way to say `reference` or `&` or `*`, you're partially right, but also completely wrong

What `quote` does is set, in this case, `y` to the _literal symbol_ of `x`. So `y = X` where `X` represents the symbol of `x` (variable symbols are designated through with uppercase)

Its a way of saying--essentially--DON'T EVALUATE THIS:

```
>> (+ 1 2 3)
6 ; obviously, we've established this
>> '(+ 1 2 3)
(+ 1 2 3) ; wha-?
>> '(* 1 2 (+ 3 4 "strawberry") a b)
(* 1 2 (+ 3 4 "strawberry") A B)
; even though this would usually cause an error
; as you can't add Strings and Numbers, because it
; doesn't evaluate, there's no issue 
>> (eval '(* 1 2 "peaches"))
--ERROR
; this DOES error, because you EVALUATE it
```

__Escapes__

There's both a shortcut for quoting AND unquoting:

```
>> '(a b c)
(A B C)
>> '(a b ,c) ; note the comma next to c
(A B nil) ; the comma negated the quote!
>> ('a 'b c)
(A B nil) ; equivalent to previous 
```

__Back to Theory__

Now is where the design of S-Expressions really comes into play. I'm going to say something, and it might not make sense... but In Jester Script, _code is data and data is code_. The very code you write can be treated as a variable. Actually, in my implementation of Jester Script, source code is represented by Lists of Objects. 

Let me explain just what I mean

```
>> (set expression ()) ; set to empty list
()
>> (append 1 expression)
1
>> (append 2 expression)
2
>> expression
(1 2)
>> (append '+ expression)
+
>> expression
(+ 1 2) ; seem familiar?
>> (eval expression)
3
```

You just constructed addition between two numbers by pushing some symbols to a List, because _code is data and data is code_

__Macros__

Macros are the final peice of the S-Expression puzzle--but they are different to the ones you know from Rust, C, or C++

In Jester, just like anything else, Macros are a part of the langauge

For example, the base Jester Script provides the following `loop`:

```
(set i 0)
(set sum 0)

; sum of integers [0 10)
(loop (< i 10)
	(incr sum i)
	(incr i 1))
```

This is pretty verbose, so lets think of a way to express a more concise loop:

```
(set sum 0)
(for i in 0 to 10
	(incr sum i))
```

With macros, it is possible to create something like this--without special cases or text-based manipulations:

```
(defmacro* for (it in min to max body)
	(let (res (gen-sym))
		'(progn
			(set ,it ,min)
			(loop (< ,it ,max)
				(set ,res (apply do ,body))
				(incr ,it 1)
				,res))))
```

Lets start from the top

`defmacro*` is the same as `defmacro`, only that it accepts a variable amount of arguments:

```
>> (defun* test (a b) b)
TEST
>> (test "a" "b" "c")
("b" "c") 
; extra arguments "b" and "c" 
; were folded into a List and stored in b
>> (test "a" "b")
("b") 
; "b" still folded into one-element list
```

`gen-sym` generates a unique symbol dynamically:

```
>> (gen-sym)
G#0
>> (gen-sym)
G#1
>> (gen-sym)
G#2
```

A unique symbol, therefore, is stored in `res`

Continuing on, the `let` expression evaluates with `res` set to the new symbol

We are evaluating a `'(progn ...)`, meaning we're returning `progn` and its arguments UNEVALUATED, save for, of course, the `,` escapes

Using the special function `macro-expand`, we can visualize what this macro will actually looks like:

```
>> (set sum 0)
0
>> (macro-expand (for i in 0 to 10 (incr sum i)))
(DO 
	(SET I 0) 
	(LOOP (< I 10) 
		(SET G#123 (APPLY DO ((INCR SUM I)))) 
		(INCR I 1) 
		G#123))
```

Kind of beautiful, isn't it?

There's a lot more to know about Lisp and Macros. For a good starting point address: https://stackoverflow.com/questions/267862/what-makes-lisp-macros-so-special
