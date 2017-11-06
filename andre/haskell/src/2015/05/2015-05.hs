import Data.List

badCombination :: String -> Bool
badCombination s = any (`isInfixOf` s) ["ab", "cd", "pq", "xy"]

doubleLetter :: String -> Bool
doubleLetter = any ((>=2) . length) . group

hasThreeVowels :: String -> Bool
hasThreeVowels s = length (filter vowel s) >= 3
    where vowel c = c `elem` "aeiou"

nice :: String -> Bool
nice = and . sequence [hasThreeVowels, doubleLetter, not . badCombination]

gap :: String -> Bool
gap xxs@(x:_:y:_)
  | x == y = True
  | otherwise = gap $ tail xxs
gap _ = False

pair :: String -> Bool
pair xxs@(x:y:xs)
  | [x, y] `isInfixOf` xs = True
  | otherwise = pair $ tail xxs
pair _ = False

nice' :: String -> Bool
nice' s = pair s && gap s

solveOne :: [String] -> Int
solveOne = length . filter nice

solveTwo :: [String] -> Int
solveTwo = length . filter nice'

main :: IO ()
main = do
    input <- readFile "inputs/2015-05"
    print . solveOne $ lines input
    print . solveTwo $ lines input
