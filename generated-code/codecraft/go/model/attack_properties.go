package model

import "io"
import . "trans_gen_test/stream"

type AttackProperties struct {
    AttackRange int32
    Damage int32
    CollectResource bool
}

func NewAttackProperties(attackRange int32, damage int32, collectResource bool) AttackProperties {
    return AttackProperties {
        AttackRange: attackRange,
        Damage: damage,
        CollectResource: collectResource,
    }
}

func ReadAttackProperties(reader io.Reader) AttackProperties {
    var attackRange int32
    attackRange = ReadInt32(reader)
    var damage int32
    damage = ReadInt32(reader)
    var collectResource bool
    collectResource = ReadBool(reader)
    return AttackProperties {
        AttackRange: attackRange,
        Damage: damage,
        CollectResource: collectResource,
    }
}

func (attackProperties AttackProperties) Write(writer io.Writer) {
    attackRange := attackProperties.AttackRange
    WriteInt32(writer, attackRange)
    damage := attackProperties.Damage
    WriteInt32(writer, damage)
    collectResource := attackProperties.CollectResource
    WriteBool(writer, collectResource)
}