module Day3 (solution) where

import Data.Char (isLower, ord)
import Data.List (intersect, nub)
import Data.List.Split (chunksOf)

score :: [Char] -> Int
score = sum . map _score
    where
        _score :: Char -> Int
        _score c
          | isLower c = (ord c) - 96
          | otherwise = (ord c) - 38

propertySum :: ([String] -> [[String]]) -> [String] -> Int
propertySum compartments = sum . map (score . nub . foldl1 intersect) . compartments

part1 :: [String] -> Int
part1 = propertySum compartments
    where 
        compartments :: [String] -> [[String]]
        compartments rucksacks = map (\r -> chunksOf (div (length r) 2) r) rucksacks

part2 = propertySum compartments
    where
        compartments :: [String] -> [[String]]
        compartments = chunksOf 3

solution :: IO (Int, Int)
solution = do
    input <- readFile "input/day3.txt"
    let rucksacks = lines input
    return (part1 rucksacks, part2 rucksacks)

