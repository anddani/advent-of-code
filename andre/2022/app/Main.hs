module Main where

import System.Environment (getArgs)
import qualified Day1 as D1
import qualified Day2 as D2
import qualified Day3 as D3
import qualified Day4 as D4

getDay :: String -> IO (Int, Int)
getDay "1" = D1.solution
getDay "2" = D2.solution
getDay "3" = D3.solution
getDay "4" = D4.solution
getDay x = error $ "Unknown argument: " ++ x

main :: IO ()
main = do
    args <- getArgs
    (part1, part2) <- getDay $ head args
    putStrLn $ "Part1: " ++ (show part1)
    putStrLn $ "Part2: " ++ (show part2)

