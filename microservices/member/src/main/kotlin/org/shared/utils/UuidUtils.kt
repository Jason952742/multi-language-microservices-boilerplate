package org.shared.utils

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

    fun uuidTobase64() {
        val uuid = UUID.randomUUID()
        val encodedString = encodeUUID(uuid)
        println(uuid)
        println("Encoded UUID: $encodedString")

        val decodedUUID = decodeUUID(encodedString)
        println(encodedString)
        println("Decoded UUID: $decodedUUID")
    }

    /**
     *  Convert UUID to Base64
     *
     * @sample uuidTobase64
     */
    fun encodeUUID(uuid: UUID): String {
        val uuidBytes = toBytes(uuid)
        val encodedBytes = Base64.getUrlEncoder().encode(uuidBytes)
        return removePadding(String(encodedBytes))
    }

    fun decodeUUID(encodedString: String): UUID {
        val decodedBytes = Base64.getUrlDecoder().decode(addPadding(encodedString))
        val uuidBytes = toUUIDBytes(decodedBytes)
        return toUUID(uuidBytes)
    }

    private fun removePadding(encodedString: String): String {
        return encodedString.removeSuffix("==")
    }

    private fun addPadding(encodedString: String): String {
        val paddingLength = encodedString.length % 4
        return encodedString + "=".repeat(paddingLength)
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
