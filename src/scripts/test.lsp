(defmacro incr (a b)
	'(set ,a (+ ,a ,b)))

(defmacro* for (var in [min max] body)
	'(do
		(set ,var ,min)
		(loop (< ,var ,max)
			(apply do ,body)
			(incr ,var 1))))

(defmacro* for-each (var in lst body)
	(let (i (gen-sym))
		'(let (,var nil)
			(set ,i 0)
			(loop (< ,i (len ,lst))
				(set ,var (nth ,i ,lst))
				(incr ,i 1)
				(apply do ,body)))))	
		
(defun* printf (frtm args)
    (println (apply format frmt args)))

(defmacro car (lst)
	(nth 0 lst))

(defmacro cdr (lst)
	(let (cpy lst)
		(remove 0 cpy)
		cpy))

(defun range (min max)
	(let (col ())
		(for i in [min max]
			(append i col))
			col))

(defmacro take (n lst)
	'(let (c ())
		(for i in [0 ,n]
			(append (nth i lst) c))
			c))

(defun slice (m n lst)
	(let (c ())
		(for i in [m n]
			(append (nth i lst) c))
			c))

(defun skip (n lst)
	(let (c ())
		(for i in [n (len lst)]
			(append (nth i lst) c))
			c))

;(take 3 (range 10))
;(take 1 (skip 1 (range 10))) => (. take 1 skip 1 (range 10))
;(let (c (take 2 comp))
;			(append (. (skip 2 comp)) c))

(defun* . (comp)
	(println "cur: " 'comp)
	(if (= (len comp) 1)
		(car comp)
		(let (c (take 2 comp))
			(println "skip " (skip 2 comp))
			(append (apply . (skip 2 comp)) c)
			c)))

(defmacro* . (comp)
	(println "cur: " comp)
	'(if (= (len ,comp) 1)
		(car ,comp)
		(let (c (take 2 ,comp))
			(append (apply . (skip 2 comp)) c)
			c)))

