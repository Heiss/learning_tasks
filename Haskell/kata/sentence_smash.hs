module Codewars.Kata.Smash where

smash :: [String] -> String
-- smash = unwords
smash = foldr ownJoin ""
    where ownJoin a b = a ++ sep b
          sep b | null b = b
                | not(null b) = " " ++ b
                