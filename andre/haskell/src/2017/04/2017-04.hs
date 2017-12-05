import Data.List

countUnique :: [[String]] -> Int
countUnique = length . filter (\x -> length x == length (nub x))

solveOne :: [[String]] -> Int
solveOne = countUnique

solveTwo :: [[String]] -> Int
solveTwo = countUnique . map (map sort)

main :: IO ()
main = do
    input <- readFile "inputs/2017-04"
    let password = map words $ lines input
    print . solveOne $ password
    print . solveTwo $ password
