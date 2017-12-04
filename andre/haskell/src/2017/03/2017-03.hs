import Data.List
import Data.Maybe (mapMaybe, fromJust)
import qualified Data.Map as M

-- Data
type Pos = (Int, Int)
type Grid = M.Map Pos Int

data SpiralPos = SpiralPos Pos (Int, Int)
    deriving (Show)

initialSpiralPos = SpiralPos (0, 0) (0, -1)
initialGrid = M.singleton (0, 0) 1

tupleAdd :: Pos -> Pos -> Pos
tupleAdd (x1, y1) (x2, y2) = (x1 + x2, y1 + y2)

next :: SpiralPos -> SpiralPos
next (SpiralPos (x, y) (dx, dy))
  | x == y || (x < 0 && x == (-y)) || (x > 0 && x == 1 - y) = SpiralPos (x - dy, y + dx) (-dy, dx)
  | otherwise = SpiralPos (x + dx, y + dy) (dx, dy)

-- Part 1
getSteps :: Int -> Maybe (Int, Int)
getSteps input = find ((>= input) . snd) $ zip [0, 2..] [x^2 | x <- [1, 3..]]

solveOne :: Int -> Int
solveOne input =
    let Just (steps, edge) = getSteps input
     in steps - (edge - input) `mod` steps

-- Part 2
surroundingPositions :: (Int, Int) -> [(Int, Int)]
surroundingPositions pos = map (tupleAdd pos) [(a, b) | a <- [-1, 0, 1], b <- [-1, 0, 1]]

sumOfSurrounding :: Grid -> SpiralPos -> Int
sumOfSurrounding grid (SpiralPos pos _) =
    sum $ mapMaybe (`M.lookup` grid) (surroundingPositions pos)

uzumaki :: Grid -> SpiralPos -> [Int]
uzumaki grid spiralPos =
    let (SpiralPos pos _) = spiralPos
        val = sumOfSurrounding grid spiralPos
        nextGrid = M.insert pos val grid
     in val : uzumaki nextGrid (next spiralPos)

solveTwo :: Int -> Int
solveTwo input =
    fromJust $ find (> input) $ uzumaki initialGrid initialSpiralPos

main :: IO ()
main = do
    input <- readFile "inputs/2017-03"
    print $ solveOne (read input)
    print $ solveTwo (read input)
