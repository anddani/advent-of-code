module Day10 (solution) where

import Data.List.Split (chunksOf)

data Op = NoOp | Addx Int
    deriving (Show)

parseOp :: String -> Op
parseOp "noop" = NoOp
parseOp ('a':'d':'d':'x':' ':num) = Addx $ read num

apply :: Op -> [Int]
apply NoOp = [0]
apply (Addx x) = [0, x]

renderLine :: [Int] -> String
renderLine = zipWith pixel [0..]
    where
        pixel :: Int -> Int -> Char
        pixel index x = if abs (x - index) <= 1 then '#' else '.'

part1 :: [Int] -> Int
part1 signals = sum [i * (signals!!(i-1)) | i <- [20, 60, 100, 140, 180, 220]]

part2 :: [Int] -> [String]
part2 = map renderLine . chunksOf 40

solution :: IO (Int, Int)
solution = do
    input <- readFile "input/day10.txt"
    let ops = parseOp <$> lines input
    let signals = scanl (+) 1 $ concatMap apply ops
    putStrLn $ unlines (part2 signals)
    return (part1 signals, 0)

