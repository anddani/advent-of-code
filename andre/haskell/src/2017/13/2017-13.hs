import Data.Maybe
import Data.List
import Data.List.Split

mapFst :: (a -> c) -> (a, b) -> (c, b)
mapFst f (x, y) = (f x, y)

isCaught :: (Int, Int) -> Bool
isCaught (picoSecond, size) =
    let cycleSize = 2 * (size - 1)
        pos = picoSecond `mod` cycleSize
     in ([0..(size-1)] ++ reverse [1..(size-2)]) !! pos == 0

severity :: (Int, Int) -> Int
severity x@(picoSecond, size) =
     if isCaught x then picoSecond * size else 0

solveOne :: [(Int, Int)] -> Int
solveOne = sum . map severity

addDelay :: Int -> [(Int, Int)] -> [(Int, Int)]
addDelay delay = map (mapFst (+delay))

solveTwo :: [(Int, Int)] -> Int
solveTwo scanner = fromJust $ elemIndex True $ map (not . any isCaught . (`addDelay` scanner)) [0..]

main :: IO ()
main = do
    input <- readFile "inputs/2017-13"
    let scanner = map ((\(x:y:_) -> (read x, read y)) . splitOn ": ") (lines (init input))
    print $ solveOne scanner
    print $ solveTwo scanner
