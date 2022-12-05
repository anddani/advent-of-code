module Main where

import System.Environment (getArgs)
import qualified Day1 as D1

getDay :: String -> IO (Int, Int)
getDay "1" = D1.day1
getDay x = error $ "Unknown argument: " ++ x

main :: IO ()
main = do
    args <- getArgs
    (part1, part2) <- getDay $ head args
    putStrLn $ "Part1: " ++ (show part1)
    putStrLn $ "Part2: " ++ (show part2)

