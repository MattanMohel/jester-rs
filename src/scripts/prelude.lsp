(defmacro incr (a b)
	'(set ,a (+ ,a ,b)))

(defmacro* for (it in min to max body)
	(let (res (gen-sym))
		'(do
			(set ,it ,min)
			(loop (< ,it ,max)
				(set ,res (apply do ,body))
				(incr ,it 1)
				,res))))

(defmacro* for-each (var in list body)
	(let (it  (gen-sym))
		'(do
			(set ,it 0)
			(loop (< ,it (len ,list))
				(set ,var (nth ,it ,list))
				(incr ,it 1)
				(apply do ,body)))))

(defun range (beg to end)
	(let (c ())
		(for i in beg to end
			(append i c)
			c)))

(defun take (end list)
	(let (c ())
		(for i in 0 to end
			(append (nth i list) c)
			c)))

(defun skip (beg list)
	(let (c ())
		(for i in beg to (len list)
			(append (nth i list) c)
			c)))

(defun slice (beg end list)
	(let (c ())
		(for i in beg to end
			(append (nth i list) c)
			c)))

(defun filter (cond list)
	(let (c ())
		(for i in 0 to (len list)
			(if (cond (nth i list))
				(append (nth i list) c))
			c)))