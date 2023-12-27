package org.shared.common.base

import java.time.LocalDateTime

interface MomentInterval {
    val start_time: LocalDateTime
    val end_time: LocalDateTime?
    val priority: Int
    val total: Int
}

interface MomentInterDetail {

}

interface PPT {

}
interface Desc
interface Role

interface Party : org.shared.common.base.PPT
interface PartyDescription : org.shared.common.base.Desc
interface PartyRole : org.shared.common.base.Role

interface Place : org.shared.common.base.PPT
interface PlaceDescription : org.shared.common.base.Desc
interface PlaceRole : org.shared.common.base.Role

interface Thing : org.shared.common.base.PPT
interface ThingDescription : org.shared.common.base.Desc
interface ThingRole: org.shared.common.base.Role
