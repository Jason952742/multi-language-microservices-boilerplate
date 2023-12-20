package org.acme.utils

import kotlin.math.roundToInt

object CaptchaUtils {

    /**
     * four-digit CAPTCHA
     *
     * @return
     */
    fun generator4Code(): String = (Math.random() * 8999 + 1000).roundToInt().toString()

    fun generator6Code(): String = (Math.random() * 899999 + 100000).roundToInt().toString()
}
