package org.shared.utils

import java.security.MessageDigest

import javax.crypto.Cipher
import javax.crypto.spec.SecretKeySpec
import org.apache.commons.codec.binary.Base64

/**
 * Encrypt and decrypt arbitrary bytes and strings with user-supplied keys using AES/ECB/PKCS5Padding
 */
object EncryptionUtils {
    private var key: String = "default-key"

    const val salt = "salt-value-multi-uxKe89Mn6p"
    private val spec by lazy {
        return@lazy keyToSpec(key)
    }

    fun setKey(k: String) {
        key = k
    }

    fun String.encrypt(): String = encryptBytes(this.toByteArray(Charsets.UTF_8))

    private fun encryptBytes(value: ByteArray): String = run {
        val cipher: Cipher = Cipher.getInstance("AES/ECB/PKCS5Padding")
        cipher.init(Cipher.ENCRYPT_MODE, spec)
        Base64.encodeBase64String(cipher.doFinal(value))
    }

    fun String.decrypt(throwOnError: Boolean = false) = String(decryptBytes(this, throwOnError))

    private fun decryptBytes(encryptedValue: String, throwOnError: Boolean = false): ByteArray = try {
        val cipher: Cipher = Cipher.getInstance("AES/ECB/PKCS5PADDING")
        cipher.init(Cipher.DECRYPT_MODE, spec)
        cipher.doFinal(Base64.decodeBase64(encryptedValue))
    } catch (e: Exception) {
        if (throwOnError) throw e
        else byteArrayOf()
    }

    private fun keyToSpec(key: String): SecretKeySpec = run {
        var keyBytes = (salt + key).toByteArray(Charsets.UTF_8)
        val sha: MessageDigest = MessageDigest.getInstance("SHA-1")
        keyBytes = sha.digest(keyBytes)
        keyBytes = keyBytes.copyOf(16)
        SecretKeySpec(keyBytes, "AES")
    }
}
