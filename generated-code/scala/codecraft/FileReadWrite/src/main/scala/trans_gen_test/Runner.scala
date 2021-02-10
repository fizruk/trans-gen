import java.io._
import trans_gen_test.util.StreamUtil

object Runner extends App {
    val inputFile = args(0)
    val outputFile = args(1)
    val repeat = Integer.parseInt(args(2))

    for (i <- 1 to repeat) {
        val inputStream: InputStream = new BufferedInputStream(new FileInputStream(inputFile))
        val input: trans_gen_test.codegame.MessageGameModel = trans_gen_test.codegame.MessageGameModel.readFrom(inputStream)
        if (repeat == 1) {
            println(input)
        }
        val outputStream: OutputStream = new BufferedOutputStream(new FileOutputStream(outputFile))
        input.writeTo(outputStream)
        outputStream.flush()
        outputStream.close()
    }
}