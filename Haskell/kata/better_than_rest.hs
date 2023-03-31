module HowGoodAreYou where

betterThanAverage :: [Int] -> Int -> Bool
betterThanAverage = (<) . avg
     where avg xs = div (sum xs) (length xs)
