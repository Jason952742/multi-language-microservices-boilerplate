package org.shared.utils

import java.nio.ByteBuffer
import java.util.*
import kotlin.math.roundToInt

object CaptchaUtils {

    /**
     * four-digit CAPTCHA
     *
     * @return
     */
    fun generator4Code(): String = (Math.random() * 8999 + 1000).roundToInt().toString()

    fun generator6Code(): String = (Math.random() * 899999 + 100000).roundToInt().toString()

    fun generatorShortUUID(uuid: UUID): String {
        // UUID to byteBuffer
        val byteBuffer = ByteBuffer.wrap(ByteArray(16))
        byteBuffer.putLong(uuid.mostSignificantBits)
        byteBuffer.putLong(uuid.leastSignificantBits)
        val uuidBytes = byteBuffer.array()

        // Converting byte arrays to strings using Base64 encoding
        val shortenedUUID = Base64.getUrlEncoder().withoutPadding().encodeToString(uuidBytes)

        return shortenedUUID
    }

}
