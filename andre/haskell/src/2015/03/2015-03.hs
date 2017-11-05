import Data.List
import Data.Set as S hiding(foldl')

data House = House Int Int
    deriving (Eq, Ord)

deliver :: House -> Char -> House
deliver (House x y) '<' = House (x-1) y
deliver (House x y) '^' = House x (y-1)
deliver (House x y) '>' = House (x+1) y
deliver (House x y) 'v' = House x (y+1)

initialPos = House 0 0

everyOther :: [a] -> [a]
everyOther (x:y:xs) = x : everyOther xs
everyOther _ = []

visit :: (House, Set House) -> Char -> (House, Set House)
visit (curr, houses) dir =
    let next = deliver curr dir
     in (next, S.insert curr houses)

housesVisited :: String -> Set House
housesVisited path = snd $ foldl' visit (initialPos, S.singleton initialPos) path

solveOne :: String -> Int
solveOne path = S.size $ housesVisited path

solveTwo :: String -> Int
solveTwo path =
    let santaHouses = housesVisited $ everyOther path
        roboHouses  = housesVisited $ everyOther $ drop 1 path
     in S.size $ S.union santaHouses roboHouses

main :: IO ()
main = do
    input <- readFile "inputs/2015-03"
    print $ solveOne input
    print $ solveTwo input
