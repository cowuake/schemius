(begin
    (define (f x) (* x 2))
    (f 3)

    ; (define factorial-recursive
    ;     (lambda (n)
    ;         (cond ((= n 0) 1)
    ;               ((> n 0) (* n (factorial-recursive (- n 1)))))))

    ; (define factorial-tail-recursive
    ;     (lambda (n)
    ;         (define aux
    ;             (lambda (n acc)
    ;                 (cond
    ;                     ((= n 0) acc)
    ;                     ((> n 0) (aux (- n 1) (* n acc))))))
    ;     (aux n 1)))
)
