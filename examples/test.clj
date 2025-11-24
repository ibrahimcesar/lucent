; Clojure example file
(ns calculator.core)

;; Public function for addition
(defn add [a b]
  (+ a b))

;; Public function for multiplication
(defn multiply [a b]
  (* a b))

;; Private function
(defn- validate-number [n]
  (if (number? n)
    true
    false))

;; Function with multiple arities
(defn greet
  ([] (greet "World"))
  ([name] (str "Hello, " name "!")))
