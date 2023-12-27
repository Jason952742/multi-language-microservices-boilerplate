package org.shared.utils

import io.github.novacrypto.bip39.MnemonicGenerator
import io.github.novacrypto.bip39.Words
import io.github.novacrypto.bip39.wordlists.English
import java.security.SecureRandom

object MnemonicUtil {
    fun generateMnemonic(): String {
        val sb = StringBuilder()
        val entropy = ByteArray(Words.TWELVE.byteLength())
        SecureRandom().nextBytes(entropy)
        MnemonicGenerator(English.INSTANCE)
            .createMnemonic(entropy) { s: CharSequence? -> sb.append(s) }
        return sb.toString()
    }
}
