import qualified Trans
import Control.Monad
import Data.Binary.Get (runGet)
import Data.Binary.Put (runPut)
import qualified Data.ByteString.Lazy as BS
import qualified System.Environment
import Codegame.MessageGameModel

main :: IO ()
main = do
    args <- System.Environment.getArgs
    let inputFile = args !! 0
    let outputFile = args !! 1
    let repeat :: Int = read $ args !! 2
    forM_ [1..repeat] $ \_ -> do
        contents <- BS.readFile inputFile
        let model :: MessageGameModel = runGet Trans.read contents
        when (repeat == 1) $ do
            print model
        BS.writeFile outputFile (runPut $ Trans.write model)