namespace TransGenTest

open System
open System.IO
open System.Net.Sockets

module Runner =

    [<EntryPoint>]
    let main argv =
        let host = argv.[0]
        let port = argv.[1] |> Int32.Parse
        let stdout = argv.[2] |> Boolean.Parse

        use client = new TcpClient(host, port)
        let stream = new BufferedStream(client.GetStream())

        let reader = new BinaryReader(stream)
        let writer = new BinaryWriter(stream)

        while reader.ReadBoolean() do
            let input = Example.readFrom(reader)
            if stdout then
                Console.WriteLine(input)
            input.writeTo(writer)
            writer.Flush()

        0