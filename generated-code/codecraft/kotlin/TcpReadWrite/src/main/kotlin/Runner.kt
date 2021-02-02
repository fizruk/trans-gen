import java.io.*
import java.net.Socket

fun main(args: Array<String>) {
    if (args.size != 2) {
        throw RuntimeException("Pass host and port as parameters")
    }
    val host = args[0]
    val port = Integer.parseInt(args[1])

    val socket = Socket(host, port)
    socket.setTcpNoDelay(true)

    val inputStream: InputStream = BufferedInputStream(socket.getInputStream())
    val input: model.PlayerView = model.PlayerView.readFrom(inputStream)

    println(input)

    val outputStream: OutputStream = BufferedOutputStream(socket.getOutputStream())
    input.writeTo(outputStream)
    outputStream.flush()

    socket.close()
}