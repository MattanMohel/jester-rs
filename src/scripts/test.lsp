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
  (let (i min c ())
    (loop (< i max)
      (append (map i) c)
      (++ i))
    c))      
