import Data.List.Split
import Data.List

rotate' :: Int -> [a] -> [a]
rotate' 0 xs = xs
rotate' n xs = rotate' (n-1) (last xs : init xs)

rotate :: Int -> [a] -> [a]
rotate _ [] = []
rotate n xs = zipWith const (drop n (cycle xs)) xs

knot :: Int -> [Int] -> [Int]
knot n xs = reverse (take n xs) ++ drop n xs

solve :: (Int, [Int]) -> Int -> (Int, [Int])
solve (skipSize, curr) x =
    let new = rotate (x + skipSize) (knot x curr)
     in (skipSize + 1, new)

rotateBack :: [Int] -> [Int] -> [Int]
rotateBack ls =
    rotate' (sum (take (length ls) [0..]) + sum ls) 

solveOne :: [Int] -> Int
solveOne input = product . take 2 . rotateBack input . snd $ foldl' solve (0, [0..255]) input

main :: IO ()
main = do
    input <- readFile "inputs/2017-10"
    let l = map (read :: String -> Int) $ splitOn "," input
    print $ solveOne l
    -- print $ solveTwo input
