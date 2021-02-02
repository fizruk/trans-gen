import Model
import qualified Trans
import Data.Binary.Get (runGet)
import Data.Binary.Put (runPut)
import qualified Data.ByteString.Lazy as BS
import qualified System.Environment
import qualified Control.Exception as CE
import Network.Socket
import Network.Socket.ByteString.Lazy as SBS

runTCPClient :: HostName -> ServiceName -> (Socket -> IO a) -> IO a
runTCPClient host port client = withSocketsDo $ do
    addr <- resolve
    CE.bracket (open addr) close client
  where
    resolve = do
        let hints = defaultHints { addrSocketType = Stream }
        head <$> getAddrInfo (Just hints) (Just host) (Just port)
    open addr = do
        sock <- socket (addrFamily addr) (addrSocketType addr) (addrProtocol addr)
        connect sock $ addrAddress addr
        return sock

main :: IO ()
main = do
    args <- System.Environment.getArgs
    let host = args !! 0
    let port = args !! 1
    runTCPClient host port $ \socket -> do
        contents <- SBS.getContents socket
        let model :: Example = runGet Trans.read contents
        print model
        SBS.sendAll socket (runPut $ Trans.write model)