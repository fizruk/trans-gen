<?php

require_once 'Codegame/MessageGameModel.php';
require_once 'Stream.php';

class FileInputStream extends InputStream
{
    private $stream;
    function __construct($path)
    {
        $this->stream = fopen($path, "rb");
    }
    function __destruct()
    {
        fclose($this->stream);
    }
    public function read($byteCount)
    {
        $data = fread($this->stream, $byteCount);
        assert(strlen($data) == $byteCount);
        return $data;
    }
}

class FileOutputStream extends OutputStream
{
    private $stream;
    function __construct($path)
    {
        $this->stream = fopen($path, "wb");
    }
    function __destruct()
    {
        $this->flush();
        fclose($this->stream);
    }
    public function write($bytes)
    {
        fwrite($this->stream, $bytes);
    }
    public function flush()
    {
        fflush($this->stream);
    }
}

$inputFile = $argv[1];
$outputFile = $argv[2];
$repeat = intval($argv[3]);

for ($i = 0; $i < $repeat; $i++) {
    $input = \Codegame\MessageGameModel::readFrom(new FileInputStream($inputFile));
    if ($repeat == 1) {
        print_r($input);
    }
    $input->writeTo(new FileOutputStream($outputFile));
}