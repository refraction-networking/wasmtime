;;! target = "x86_64"

(module
    (func (result i32)
	(i32.const -1)
	(i32.const -1)
	(i32.rem_u)
    )
)
;;    0:	 55                   	push	rbp
;;    1:	 4889e5               	mov	rbp, rsp
;;    4:	 4883ec08             	sub	rsp, 8
;;    8:	 4c893424             	mov	qword ptr [rsp], r14
;;    c:	 b9ffffffff           	mov	ecx, 0xffffffff
;;   11:	 b8ffffffff           	mov	eax, 0xffffffff
;;   16:	 31d2                 	xor	edx, edx
;;   18:	 f7f1                 	div	ecx
;;   1a:	 89d0                 	mov	eax, edx
;;   1c:	 4883c408             	add	rsp, 8
;;   20:	 5d                   	pop	rbp
;;   21:	 c3                   	ret	
