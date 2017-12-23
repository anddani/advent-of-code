import Data.Bits (xor)
import Data.Char
import Data.List.Split
import Data.List
import Numeric (showHex)

toHex :: Int -> String
toHex x =
    let hex = showHex x ""
     in if x < 16 then '0' : hex else hex

rotate' :: Int -> [a] -> [a]
rotate' 0 xs = xs
rotate' n xs = rotate' (n-1) (last xs : init xs)

rotateBack :: [Int] -> [Int] -> [Int]
rotateBack ls =
    rotate' ((sum (take (length ls) [0..]) + sum ls) `mod` 256)

rotate :: Int -> [a] -> [a]
rotate _ [] = []
rotate n xs = zipWith const (drop n (cycle xs)) xs

knot :: Int -> [Int] -> [Int]
knot n xs = reverse (take n xs) ++ drop n xs

solve :: (Int, [Int]) -> Int -> (Int, [Int])
solve (skipSize, curr) x =
    let new = rotate (x + skipSize) (knot x curr)
     in (skipSize + 1, new)

executeKnots :: [Int] -> [Int]
executeKnots input = rotateBack input . snd $ foldl' solve (0, [0..255]) input

solveOne :: [Int] -> Int
solveOne = product . take 2 . executeKnots

solveTwo :: [Int] -> String
solveTwo input =
    let afterRounds = executeKnots . concat $ replicate 64 input
        sparseHash = chunksOf 16 afterRounds
        denseHash = foldl1 xor <$> sparseHash
        hex = toHex <$> denseHash
     in concat hex

main :: IO ()
main = do
    input <- readFile "inputs/2017-10"
    let input1 = map (read :: String -> Int) $ splitOn "," input
    print $ solveOne input1
    let input2 = map ord (init input) ++ [17, 31, 73, 47, 23]
    print $ solveTwo input2
