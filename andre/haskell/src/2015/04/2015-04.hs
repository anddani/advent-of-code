import Data.List
import qualified Data.Digest.Pure.MD5 as M
import qualified Data.ByteString.Lazy.Char8 as BC

hash :: String -> Int -> String
hash secretKey n = show . M.md5 . BC.pack $ secretKey ++ show n

solve :: String -> (String -> Bool) -> Int
solve secretKey prefix = 
    case findIndex prefix (map (hash secretKey) [1..]) of
      Just index -> index + 1
      Nothing    -> -1

main :: IO ()
main = do
    input <- readFile "inputs/2015-04"
    print $ solve (init input) $ isPrefixOf (replicate 5 '0')
    print $ solve (init input) $ isPrefixOf (replicate 6 '0')
