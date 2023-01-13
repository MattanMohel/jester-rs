(defmacro incr (a b)
	'(set ,a (+ ,a ,b)))

(defmacro* for (it in min to max body)
	(let (res (gen-sym))
		'(do
			(set ,it ,min)
			(loop (< ,it ,max)
				(set ,res (apply do body))
				(incr ,it 1)
				,res))))