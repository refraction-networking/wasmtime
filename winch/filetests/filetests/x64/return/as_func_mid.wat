;;! target = "x86_64"
(module
  (func $dummy)
  (func (export "as-func-mid") (result i32)
   (call $dummy) (return (i32.const 2)) (i32.const 3)
  )
)
  
;;    0:	 55                   	push	rbp
;;    1:	 4889e5               	mov	rbp, rsp
;;    4:	 4883ec08             	sub	rsp, 8
;;    8:	 4c893424             	mov	qword ptr [rsp], r14
;;    c:	 4883c408             	add	rsp, 8
;;   10:	 5d                   	pop	rbp
;;   11:	 c3                   	ret	
;;
;;    0:	 55                   	push	rbp
;;    1:	 4889e5               	mov	rbp, rsp
;;    4:	 4883ec08             	sub	rsp, 8
;;    8:	 4c893424             	mov	qword ptr [rsp], r14
;;    c:	 4883ec08             	sub	rsp, 8
;;   10:	 e800000000           	call	0x15
;;   15:	 4883c408             	add	rsp, 8
;;   19:	 b802000000           	mov	eax, 2
;;   1e:	 4883c408             	add	rsp, 8
;;   22:	 5d                   	pop	rbp
;;   23:	 c3                   	ret	
