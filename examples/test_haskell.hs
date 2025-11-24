-- Haskell example file
module Factorial where

-- | Calculate factorial recursively
factorial :: Integer -> Integer
factorial 0 = 1
factorial n = n * factorial (n - 1)

-- | Check if a number is even
isEven :: Int -> Bool
isEven n = n `mod` 2 == 0

-- | Calculate sum of list
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
