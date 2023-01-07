(defmacro incr (a b)
	'(set ,a (+ ,a ,b)))

(defmacro* for (var in [min max] body)
	'(do
		(set ,var ,min)
		(loop (< ,var ,max)
			(apply do ,body)
			(incr ,var 1))))

; (defmacro* for-each (var in lst body)
; 	(let (i (gen-sym))
; 		'(let (,var nil)
; 			(set ,i 0)
; 			(loop (< ,i (len ,lst))
; 				(set ,var (nth ,i ,lst))
; 				(incr ,i 1)
; 				(apply do ,body)))))	
		
; (defun* printf (frmt args)
;     (println (apply format frmt args)))

; (defmacro car (lst)
; 	(nth 0 lst))

; (defmacro cdr (lst)
; 	(let (cpy lst)
; 		(remove 0 cpy)
; 		cpy))

; (defun range (min max)
; 	(let (col ())
; 		(for i in [min max]
; 			(append i col))
; 			col))

; (defun take (n lst)
; 	(let (c ())
; 		(for i in [0 ,n]
; 			(append (nth i lst) c))
; 			c))

; (defun slice (m n lst)
; 	(let (c ())
; 		(for i in [m n]
; 			(append (nth i lst) c))
; 			c))

; (defun rev (lst)
;     (let (c ())
;         (for i in [0 (len lst)]
;             (append (nth (- (len lst) i)) c))
;         c))

; (defun skip (n lst)
; 	(let (c ())
; 		(for i in [n (len lst)]
; 			(append (nth i lst) c))
; 			c))

nil