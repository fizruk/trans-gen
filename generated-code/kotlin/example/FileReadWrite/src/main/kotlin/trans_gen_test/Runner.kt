import java.io.*

fun main(args: Array<String>) {
    val inputFile = args[0]
    val outputFile = args[1]
    val repeat = Integer.parseInt(args[2])

    for (i in 1..repeat) {
        val inputStream: InputStream = BufferedInputStream(FileInputStream(inputFile))
        val input: trans_gen_test.Example = trans_gen_test.Example.readFrom(inputStream)
        if (repeat == 1) {
            println(input)
        }
        val outputStream: OutputStream = BufferedOutputStream(FileOutputStream(outputFile))
        input.writeTo(outputStream)
        outputStream.flush()
        outputStream.close()
    }
}