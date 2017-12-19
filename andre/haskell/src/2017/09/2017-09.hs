import Control.Applicative ((<$), (<*), (*>))
import Control.Monad ((>>))
import Data.Either.Unwrap
import Data.List
import Text.ParserCombinators.Parsec

data Stream = Garbage String | Group [Stream]
    deriving Show

parseGarbageContent :: Parser String
parseGarbageContent =
    (char '!' >> anyChar >> return "")
    <|> (:[]) <$> noneOf ">"

parseGarbage :: Parser Stream
parseGarbage =
    between (char '<') (char '>') $
        Garbage <$> (concat <$> many parseGarbageContent)

parseGroup :: Parser Stream
parseGroup =
    between (char '{') (char '}') $
        Group <$> parseStream `sepBy` char ','

parseStream :: Parser Stream
parseStream = parseGroup <|> parseGarbage

parseInput :: String -> Stream
parseInput = fromRight . parse parseStream ""

solveOne :: Int -> Stream -> Int
solveOne depth (Group gs) = depth + sum (map (solveOne (depth + 1)) gs)
solveOne _ _ = 0

solveTwo (Group gs) = sum (map solveTwo gs)
solveTwo (Garbage g) = length g

main :: IO ()
main = do
    input <- readFile "inputs/2017-09"
    let stream = parseInput (init input)
    print $ solveOne 1 stream
    print $ solveTwo stream
