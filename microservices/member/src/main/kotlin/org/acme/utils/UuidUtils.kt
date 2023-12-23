package org.acme.utils

import java.nio.ByteBuffer
import java.util.*

object UuidUtils {

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

    fun encodeUUID(uuid: UUID): String {
        val uuidBytes = toBytes(uuid)
        val encodedBytes = Base64.getUrlEncoder().encode(uuidBytes)
        return String(encodedBytes)
    }

    fun decodeUUID(encodedString: String): UUID {
        val decodedBytes = Base64.getUrlDecoder().decode(encodedString)
        val uuidBytes = toUUIDBytes(decodedBytes)
        return toUUID(uuidBytes)
    }

    private fun toBytes(uuid: UUID): ByteArray {
        val mostSigBits = uuid.mostSignificantBits
        val leastSigBits = uuid.leastSignificantBits
        val bytes = ByteArray(16)
        for (i in 0..7) {
            bytes[i] = (mostSigBits shr 8 * (7 - i)).toByte()
            bytes[8 + i] = (leastSigBits shr 8 * (7 - i)).toByte()
        }
        return bytes
    }

    private fun toUUIDBytes(bytes: ByteArray): ByteArray {
        val uuidBytes = ByteArray(16)
        for (i in 0..7) {
            uuidBytes[i] = bytes[i]
            uuidBytes[8 + i] = bytes[8 + i]
        }
        return uuidBytes
    }

    private fun toUUID(bytes: ByteArray): UUID {
        val mostSigBits = bytesToLong(bytes, 0)
        val leastSigBits = bytesToLong(bytes, 8)
        return UUID(mostSigBits, leastSigBits)
    }

    private fun bytesToLong(bytes: ByteArray, offset: Int): Long {
        var result: Long = 0
        for (i in offset until offset + 8) {
            result = result shl 8 or (bytes[i].toLong() and 0xFF)
        }
        return result
    }

}
