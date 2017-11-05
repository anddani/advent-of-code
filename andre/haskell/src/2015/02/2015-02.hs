import Data.List
import Data.List.Split

twoSmallest :: [Int] -> [Int]
twoSmallest = take 2 . sort

areaOfSmallest :: [Int] -> Int
areaOfSmallest box =
    let [a, b] = twoSmallest box
     in a * b

squareFeet :: [Int] -> Int
squareFeet box =
    let [l, w, h] = box
     in 2*l*w + 2*w*h + 2*l*h + areaOfSmallest box

smallestPerimiterOf :: [Int] -> Int
smallestPerimiterOf box =
    let [a, b] = twoSmallest box
     in 2*a + 2*b

ribbonLength :: [Int] -> Int
ribbonLength box = 
    smallestPerimiterOf box + foldl' (*) 1 box

solve :: ([Int] -> Int) -> [[Int]] -> Int
solve f = foldr ((+) . f) 0

solveOne :: [[Int]] -> Int
solveOne = solve squareFeet

solveTwo :: [[Int]] -> Int
solveTwo = solve ribbonLength

main :: IO ()
main = do
    input <- readFile "inputs/2015-02"
    let boxes = map (map read . splitOn "x") (lines input)
    print $ solveOne boxes
    print $ solveTwo boxes
