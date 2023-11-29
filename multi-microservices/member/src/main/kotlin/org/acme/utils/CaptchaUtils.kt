package org.acme.utils

import kotlin.math.roundToInt

object CaptchaUtils {

    /**
     * four-digit CAPTCHA
     *
     * @return
     */
    fun generatorCode(): String = (Math.random() * 8999 + 1000).roundToInt().toString()
}
