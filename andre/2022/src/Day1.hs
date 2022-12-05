module Day1 (solution) where

import Data.List (sortBy)
import Data.List.Split (splitOn)

elves :: String -> [Int]
elves = map (sum . map read . lines) . splitOn "\n\n"

part1 :: String -> Int
part1 = maximum . elves

part2 :: String -> Int
part2 = sum . take 3 . sortBy (flip compare) . elves

solution :: IO (Int, Int)
solution = do
    input <- readFile "input/day1.txt"
    return (part1 input, part2 input)

