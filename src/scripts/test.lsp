(defmacro += (a b)
	'(set ,a (+ ,a ,b)))

(defmacro ++ (a)
	'(+= ,a ,1))

(defmacro map (lst fun)
	(let (i (gen-sym 0))
		'(loop (< ,i (len ,lst))
		(replace ,i (,fun (nth ,i ,lst)) ,lst)
		(++ ,i)
		,lst)))

(defun apply (fun lst)
	(prepend '+ lst)
	(eval lst))

(defun collect ([min max] map)
	(let (i min 
	      col ())	  
		(loop (< i max)
		(append (map i) col)
		(++ i))
		col))

(defmacro for (var in lst body)
	(let (i (gen-sym))
		'(let (,var nil)
			(set ,i 0)
			(loop (< ,i (len ,lst))
				(set ,var (nth ,i ,lst))
				(do ,body)
				(++ ,i)))))
	
(set hash
	(("0" 11)
	 ("1" 22)
	 ("2" 33)))
	
(defun get-val (key hm)
	(for pair in hm (
		(if (= key (nth 0 pair))
			(println pair)))))

