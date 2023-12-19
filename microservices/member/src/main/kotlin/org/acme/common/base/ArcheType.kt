package org.acme.common.base

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

interface Party : PPT
interface PartyDescription : Desc
interface PartyRole : Role

interface Place : PPT
interface PlaceDescription : Desc
interface PlaceRole : Role

interface Thing : PPT
interface ThingDescription : Desc
interface ThingRole: Role
