(def (fac x)
    (if (= x 0)
        1
        (* x (fac (- x 1)))))

(println (fac 5))
