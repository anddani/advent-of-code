import Data.List
import qualified Data.Set as S

hasDuplicates :: (Ord a) => [a] -> Bool
hasDuplicates list = length list /= length set
    where set = S.fromList list

countUnique :: [[String]] -> Int
countUnique = length . filter (not . hasDuplicates)

solveOne :: [[String]] -> Int
solveOne = countUnique

solveTwo :: [[String ]] -> Int
solveTwo = countUnique . map (map sort)

main :: IO ()
main = do
    input <- readFile "inputs/2017-04"
    let password = map words $ lines input
    print . solveOne $ password
    print . solveTwo $ password
