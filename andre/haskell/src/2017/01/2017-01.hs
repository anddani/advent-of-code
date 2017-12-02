import Data.Char

split :: String -> [(Char, Char)]
split (x:xs@(y:_)) = (x, y) : split xs
split _ = []

split' :: String -> [(Char, Char)]
split' string = zipWith (\x y -> (y, string !! halfway x)) [0..] string
    where halfway offset = (`mod` len) $ len `quot` 2 + offset
          len = length string

similar :: String -> [(Char, Char)]
similar string =
    filter (\(a, b) -> a == b) (split string ++ [(a, b)])
        where a = head string
              b = last string

similar' :: String -> [(Char, Char)]
similar' string =
    filter (\(a, b) -> a == b) (split' string)

join :: [(Char, Char)] -> [Int]
join = map (digitToInt . fst)

solveOne :: String -> Int
solveOne = sum . join . similar

solveTwo :: String -> Int
solveTwo = sum . join . similar'

main :: IO ()
main = do
    input <- readFile "inputs/2017-01"
    print $ solveOne (init input)
    print $ solveTwo (init input)
