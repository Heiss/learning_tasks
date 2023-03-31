module Kata where
import Foreign.C (CWchar)

remove :: String -> String
remove (x:xs) = check x ++ remove xs
  where check '!' = ""
        check c = [c]
remove [] = "!"