import Data.List
import Data.Ord

toIntList :: String -> [Int]
toIntList = map read . words

maxMin :: [[Int]] -> [(Int, Int)]
maxMin = map (\x -> (maximum x, minimum x))

solveOne :: [String] -> Int
solveOne string = sum $ map (uncurry (-)) (maxMin matrix)
    where matrix = map toIntList string

divisible :: (Int, Int) -> Bool
divisible (a, b) = a `rem` b == 0 && a /= b

withDivisible :: [Int] -> (Int, Int)
withDivisible l = head $ filter divisible [(a, b) | a <- l, b <- l]

divisibles :: [[Int]] -> [(Int, Int)]
divisibles = map withDivisible

solveTwo :: [String] -> Int
solveTwo string = sum $ map (uncurry quot) (divisibles matrix)
    where matrix = map toIntList string

main :: IO ()
main = do
    input <- readFile "inputs/2017-02"
    print $ solveOne $ lines (init input)
    print $ solveTwo $ lines (init input)
