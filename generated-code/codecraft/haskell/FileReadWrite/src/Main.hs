import Model
import qualified Trans
import Data.Binary.Get (runGet)
import Data.Binary.Put (runPut)
import qualified Data.ByteString.Lazy as BS
import qualified System.Environment

main :: IO ()
main = do
    args <- System.Environment.getArgs
    let inputFile = args !! 0
    let outputFile = args !! 1
    contents <- BS.readFile inputFile
    let model :: PlayerView = runGet Trans.read contents
    print model
    BS.writeFile outputFile (runPut $ Trans.write model)