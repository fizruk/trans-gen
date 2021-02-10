namespace TransGenTest

open System
open System.IO

module Runner =

    [<EntryPoint>]
    let main argv =
        let inputFile = argv.[0]
        let outputFile = argv.[1]
        let repeat = argv.[2] |> int

        for i in 1..repeat do
            use inputStream = new FileStream(inputFile, FileMode.Open)
            use reader = new BinaryReader(inputStream)
            let input = Codegame.MessageGameModel.readFrom(reader)

            if repeat = 1 then
                Console.WriteLine(input)

            use outputStream = new FileStream(outputFile, FileMode.Create)
            use writer = new BinaryWriter(outputStream)
            input.writeTo(writer)

        0