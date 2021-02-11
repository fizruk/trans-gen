<?php

namespace Codegame {
    require_once 'Model/PlayerView.php';
    require_once 'Stream.php';

    /**
     * Message sent from server
     */
    abstract class ServerMessage
    {
        /**
         * Write ServerMessage to output stream
         */
        abstract function writeTo(\OutputStream $stream): void;

        /**
         * Read ServerMessage from input stream
         */
        static function readFrom(\InputStream $stream): ServerMessage
        {
            $tag = $stream->readInt32();
            if ($tag == \Codegame\ServerMessage\GetAction::TAG) {
                return \Codegame\ServerMessage\GetAction::readFrom($stream);
            }
            if ($tag == \Codegame\ServerMessage\Finish::TAG) {
                return \Codegame\ServerMessage\Finish::readFrom($stream);
            }
            if ($tag == \Codegame\ServerMessage\DebugUpdate::TAG) {
                return \Codegame\ServerMessage\DebugUpdate::readFrom($stream);
            }
            throw new Exception('Unexpected tag value');
        }
    }
}

namespace Codegame\ServerMessage {
    /**
     * Get action for next tick
     */
    class GetAction extends \Codegame\ServerMessage
    {
        const TAG = 0;
    
        /**
         * Player's view
         */
        public \Model\PlayerView $playerView;
        /**
         * Whether app is running with debug interface available
         */
        public bool $debugAvailable;
    
        function __construct(\Model\PlayerView $playerView, bool $debugAvailable)
        {
            $this->playerView = $playerView;
            $this->debugAvailable = $debugAvailable;
        }
    
        /**
         * Read GetAction from input stream
         */
        public static function readFrom(\InputStream $stream): GetAction
        {
            $playerView = \Model\PlayerView::readFrom($stream);
            $debugAvailable = $stream->readBool();
            return new GetAction($playerView, $debugAvailable);
        }
        
        /**
         * Write GetAction to output stream
         */
        public function writeTo(\OutputStream $stream): void
        {
            $stream->writeInt32(GetAction::TAG);
            $this->playerView->writeTo($stream);
            $stream->writeBool($this->debugAvailable);
        }
    }

    /**
     * Signifies end of the game
     */
    class Finish extends \Codegame\ServerMessage
    {
        const TAG = 1;
    
    
        function __construct()
        {
        }
    
        /**
         * Read Finish from input stream
         */
        public static function readFrom(\InputStream $stream): Finish
        {
            return new Finish();
        }
        
        /**
         * Write Finish to output stream
         */
        public function writeTo(\OutputStream $stream): void
        {
            $stream->writeInt32(Finish::TAG);
        }
    }

    /**
     * Debug update
     */
    class DebugUpdate extends \Codegame\ServerMessage
    {
        const TAG = 2;
    
        /**
         * Player's view
         */
        public \Model\PlayerView $playerView;
    
        function __construct(\Model\PlayerView $playerView)
        {
            $this->playerView = $playerView;
        }
    
        /**
         * Read DebugUpdate from input stream
         */
        public static function readFrom(\InputStream $stream): DebugUpdate
        {
            $playerView = \Model\PlayerView::readFrom($stream);
            return new DebugUpdate($playerView);
        }
        
        /**
         * Write DebugUpdate to output stream
         */
        public function writeTo(\OutputStream $stream): void
        {
            $stream->writeInt32(DebugUpdate::TAG);
            $this->playerView->writeTo($stream);
        }
    }
}