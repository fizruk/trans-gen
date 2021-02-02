module Model.Entity where

import Prelude hiding (id)
import qualified Trans
import Trans (Trans)
import Data.Int
import Model.EntityType (EntityType)
import Model.Vec2Int32 (Vec2Int32)

data Entity = Entity {
    id :: Int32,
    playerId :: Maybe Int32,
    entityType :: EntityType,
    position :: Vec2Int32,
    health :: Int32,
    active :: Bool }
    deriving Show

instance Trans Entity where
    read = do
        id <- Trans.read
        playerId <- Trans.read
        entityType <- Trans.read
        position <- Trans.read
        health <- Trans.read
        active <- Trans.read
        return Entity {
            id,
            playerId,
            entityType,
            position,
            health,
            active }

    write Entity {
        id,
        playerId,
        entityType,
        position,
        health,
        active } = do
            Trans.write id
            Trans.write playerId
            Trans.write entityType
            Trans.write position
            Trans.write health
            Trans.write active