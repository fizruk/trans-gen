import java.io.*

fun main(args: Array<String>) {
    if (args.size != 2) {
        throw RuntimeException("Pass input and output as parameters")
    }
    val inputFile = args[0]
    val outputFile = args[1]

    val inputStream: InputStream = BufferedInputStream(FileInputStream(inputFile))
    val input: model.Structure = model.Structure.readFrom(inputStream)

    val outputStream: OutputStream = BufferedOutputStream(FileOutputStream(outputFile))
    input.writeTo(outputStream)
    outputStream.flush()
    outputStream.close()
}