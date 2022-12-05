module Day1 (day1) where

import Data.List (sortBy)
import Data.List.Split (splitOn)

elves :: String -> [Int]
elves = map (sum . map read . lines) . splitOn "\n\n"

part1 :: String -> Int
part1 = maximum . elves

part2 :: String -> Int
part2 = sum . take 3 . sortBy (flip compare) . elves

day1 :: IO (Int, Int)
day1 = do
    input <- readFile "input/day1.txt"
    return (part1 input, part2 input)

