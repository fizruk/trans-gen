#if os(Linux)
	import Glibc
#else
	import Darwin
#endif

class TcpStream : InputStream, OutputStream {
    init(_ host: String, _ port: Int) {
        sock = socket(AF_INET, Int32(SOCK_STREAM.rawValue), 0)
        if sock == -1 {
            fatalError("Failed to create socket")
        }
        var yes: Int32 = 1
        if setsockopt(sock, Int32(IPPROTO_TCP), TCP_NODELAY, &yes, socklen_t(MemoryLayout<Int32>.size)) < 0 {
            fatalError("Failed to set TCP_NODELAY")
        }
        var hints = addrinfo()
        var servinfo: UnsafeMutablePointer<addrinfo>?
        hints.ai_family = AF_INET;
        hints.ai_socktype = Int32(SOCK_STREAM.rawValue);
        if (getaddrinfo(host, String(port), &hints, &servinfo) != 0) {
            fatalError("Failed to get addr info")
        }
        if (connect(sock, servinfo!.pointee.ai_addr, servinfo!.pointee.ai_addrlen) == -1) {
            fatalError("Failed to connect")
        }
        freeaddrinfo(servinfo)
    }

	func readBytes(_ byteCount: Int) -> [Byte] {
        var byteCount = byteCount
		var buffer = [Byte](repeating: 0x0, count: byteCount)
        var pos = 0
        while byteCount > 0 {
            let received = buffer[pos...].withUnsafeMutableBytes { recv(sock, $0.baseAddress, byteCount, 0) }
            if received < 0 {
                fatalError("Failed to read from socket")
            }
            pos += received
            byteCount -= received
        }
		return buffer
    }

	func writeBytes(_ data: [Byte]) {
        if send(sock, data, data.count, 0) < 0 {
            fatalError("Failed to write to socket")
        }
    }

    func flush() {
    }

    private let sock: Int32
}