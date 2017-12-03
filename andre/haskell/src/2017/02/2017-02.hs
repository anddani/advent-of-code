import Data.List
import Data.Ord

toIntList :: String -> [Int]
toIntList = map read . words

maxMin :: [[Int]] -> [(Int, Int)]
maxMin = map (\x -> (maximum x, minimum x))

solveOne :: [String] -> Int
solveOne string = sum $ map (\x -> maximum x - minimum x) matrix
    where matrix = map toIntList string

divisibles :: [[Int]] -> [(Int, Int)]
divisibles = map (\x -> head [(a, b) | a <- x, b <- x, a `mod` b == 0, a /= b])

solveTwo :: [String] -> Int
solveTwo string = sum $ map (uncurry quot) (divisibles matrix)
    where matrix = map toIntList string

main :: IO ()
main = do
    input <- readFile "inputs/2017-02"
    print $ solveOne $ lines (init input)
    print $ solveTwo $ lines (init input)
