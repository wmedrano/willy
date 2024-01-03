(define-module (willy open-file)
  #:export (open-file!))
(use-modules ((willy state)       #:prefix state:)
             ((willy core buffer) #:prefix buffer:)
             ((willy core window) #:prefix window:)
             ((willy core log)    #:select (log!))
             ((willy modal)       #:select (run-modal!))
             ((ice-9 ftw)         #:prefix ftw:)
             ((ice-9 textual-ports))
             ((srfi srfi-1))
             ((srfi srfi-2)))

(define (on-select-file window query file)
  (and-let* ((text   (call-with-input-file file get-string-all))
             (buffer (buffer:make-buffer #:name file
                                         #:string text
                                         #:language (language-for-file file))))
    (window:window-set-buffer! window buffer)))

(define* (open-file!)
  "Open a new file on the current window."
  (run-modal! #:modal-name "open-file"
              #:prompt     "Open File: "
              #:items      (discover-files)
              #:on-select  on-select-file))

(define* (discover-files)
  (let* ((file-name ".")
         (init      '())
         (enter?    (lambda (path stat result)
                      (not (member (basename path) '(".git" ".svn" "CVS")))))
         (down      (lambda (path stat result) result))
         (up        (lambda (path stat result) result))
         (skip      (lambda (path stat result) result))
         (error     (lambda (path stat errno result) result))
         (leaf      (lambda (path stat result)
                      (cons path result))))
    (ftw:file-system-fold enter? leaf down up skip error init file-name)))

(define* (language-for-file path)
  "Get the language for a file."
  (cond
   ((string-contains path ".rs") "rust")
   (else "")))
