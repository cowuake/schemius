(begin
    (define factorial-recursive
        (lambda (n)
            (cond ((= n 0) 1)
                  ((> n 0) (* n (factorial-recursive (- n 1)))))))

    (define factorial-tail-recursive
        (lambda (n)
            (define aux
                (lambda (n acc)
                    (cond
                        ((= n 0) acc)
                        ((> n 0) (aux (- n 1) (* n acc))))))
        (aux n 1)))

    (display "The factorial of 5 is ")
    (display (factorial-recursive 5))
    (display " while the factorial of 5000 is ")
    (display (factorial-tail-recursive 5000))
)
