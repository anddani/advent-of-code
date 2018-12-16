import qualified Data.Map as M

frequency :: (Ord a) => [a] -> [(a, Int)]
frequency xs = M.toList (M.fromListWith (+) [(x, 1) | x <- xs])

hasFrequency :: Int -> [(a, Int)] -> Bool
hasFrequency freq = not . null . filter (\(_, x) -> x == freq)

solveOne :: [String] -> Int
solveOne input = numTwo * numThree
  where
    frequencies = map frequency input
    numTwo = sum $ map (fromEnum . hasFrequency 2) frequencies
    numThree = sum $ map (fromEnum . hasFrequency 3) frequencies

diffCount :: String -> String -> Int
diffCount a b = sum $ zipWith (\x y -> fromEnum $ x /= y) a b

removeDiff :: String -> String -> String
removeDiff (a:as) (b:bs)
  | a == b = a : removeDiff as bs
  | otherwise = removeDiff as bs
removeDiff _ _ = []

solveTwo :: [String] -> String
solveTwo xs = removeDiff correct1 correct2
  where
    (correct1, correct2) = head [(a, b) | a <- xs, b <- xs, diffCount a b == 1]

main :: IO ()
main = do
    input <- readFile "inputs/2018-02"
    print $ solveOne $ lines input
    print $ solveTwo $ lines input
