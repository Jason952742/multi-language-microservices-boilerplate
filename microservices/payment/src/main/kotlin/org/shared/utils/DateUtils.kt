package org.shared.utils

import java.text.SimpleDateFormat
import java.time.*
import java.time.temporal.TemporalAdjusters
import java.util.*
import java.time.format.*
import java.time.temporal.ChronoUnit


object DateUtils {

    ///////////////////////////////////
    // format
    private val isoFmt = DateTimeFormatter.ISO_LOCAL_DATE_TIME // "yyyy-MM-ddTHH:mm:ss"
    private val isoZonedFmt = DateTimeFormatter.ISO_ZONED_DATE_TIME // 'yyyy-MM-ddTHH:mm:ss+01:00[Europe/Paris]'
    private val dateFmt = DateTimeFormatter.ofPattern("yyyy-MM-dd") // ISO_LOCAL_DATE
    private val niceDateFmt = DateTimeFormatter.ofPattern("EEEE, MMM dd, yyyy")
    private val timeFmt = DateTimeFormatter.ofPattern("HH:mm:ss")
    private val pureFmt = DateTimeFormatter.ofPattern("yyyyMMddhhmmss")

    private val dtFmtIso = SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss")
    private val dtFmtMillis = SimpleDateFormat("yyyy-MM-dd HH:mm:ss.SSS")
    private val dtFmtDefault = SimpleDateFormat("yyyy-MM-dd HH:mm:ss")
    private val dtFmtAmPm = SimpleDateFormat("yyyy-MM-dd hh:mma")
    private val dtFmtNoSec = SimpleDateFormat("yyyy-MM-dd HH:mm")
    private val dtFmtStd = SimpleDateFormat("yyyy-MM-dd'T'HH:mm")

    private val dtFmts = listOf(dtFmtIso, dtFmtMillis, dtFmtDefault, dtFmtAmPm, dtFmtNoSec, dtFmtStd)

    fun Long?.toDatetime(): LocalDateTime? = this?.let {
        val instant: Instant = Instant.ofEpochMilli(it)
        return LocalDateTime.ofInstant(instant, ZoneId.systemDefault())
    }

    fun LocalDateTime.toTimestamp(): Long {
        return this.toInstant(ZoneOffset.of("+8")).toEpochMilli()
    }

    fun date2ld(date: Date): LocalDateTime {
        val instant = date.toInstant()
        val zoneId = ZoneId.systemDefault()
        return instant.atZone(zoneId).toLocalDateTime()
    }

    // useful for wechat
    fun LocalDateTime.toPure() = pureFmt.format(this)
    fun String.toDateTime() = LocalDateTime.from(pureFmt.parse(this))

    val farFarAway: LocalDateTime = LocalDateTime.now().plusYears(20)

    /////////////////////////////////
    //

    /**
     * Compare the distance between two dates
     *
     * @param start
     * @param end
     * @return
     */
    fun daysDiff(start: LocalDateTime, end: LocalDateTime): Long =
        ChronoUnit.DAYS.between(start.toLocalDate(), end.toLocalDate())

    // java.util.Date转LocalDateTime
    fun fromUtilDate(date: java.util.Date): LocalDateTime = LocalDateTime.ofInstant(date.toInstant(), ZoneId.systemDefault())

    // LocalDateTime转java.util.Date
    fun toUtilDate(dateTime: LocalDateTime): java.util.Date = java.util.Date.from(dateTime.atZone(ZoneId.systemDefault()).toInstant())
    fun toUtilDate(date: LocalDate): java.util.Date = toUtilDate(date.atStartOfDay())
    fun toUtilDate(time: LocalTime): java.util.Date = toUtilDate(LocalDateTime.of(LocalDate.now(), time))

    /**
     * Get date range
     * @param lastDay End date
     * @return Returns a list of time periods from the start date to the end date.
     *          Full moon in the first month and from the start date to the end of the month in the second month.
     *          Third to N-1 full months, with the final Nth month being from the 1st to the end date
     */
    fun LocalDate.toRangeDate(lastDay: LocalDate): List<DateRange> {
        println("from $this to $lastDay")
        println("-----------------------")

        var currentLastDay = if (lastDay.isAfter(this.plusMonths(1).minusDays(1))) this.plusMonths(1).minusDays(1) else lastDay
        var index = 0
        val result = mutableSetOf<DateRange>()

        do {
            when (index) {
                0 -> {
                    val max = this.month.maxLength()
                    val days = Period.between(this, currentLastDay).days + 1
                    println("from $this to $currentLastDay， There are a total of $days days --- Maximum number of days in a month ${this.month} : $max,  ${if (days == max) "full" else "not full"} month")

                    result.add(DateRange(this, currentLastDay, days == max, 1.0))
                }
                else -> {
                    // Change the current start date to the day after the previous end date
                    val currentFirstDay = currentLastDay.plusDays(1)
                    val lastDayOfMonth = currentFirstDay.with(TemporalAdjusters.lastDayOfMonth())
                    currentLastDay = if (lastDay.isAfter(lastDayOfMonth)) lastDayOfMonth else lastDay

                    val max = currentFirstDay.month.maxLength()
                    val days = Period.between(currentFirstDay, currentLastDay).days + 1
                    println("from $currentFirstDay to $currentLastDay，There are a total of $days days --- Maximum number of days in a month ${currentFirstDay.month} : $max,  ${if (days == max) "full" else "not full"} month")

                    result.add(DateRange(currentFirstDay, currentLastDay, days == max, days.toDouble() / max.toDouble()))
                }
            }
            index++
        } while (currentLastDay.isBefore(lastDay))
        return result.toList()
    }

}
