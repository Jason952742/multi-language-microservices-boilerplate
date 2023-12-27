package org.multi_lang

import jakarta.enterprise.context.ApplicationScoped


@ApplicationScoped
class MyOtherBean {
    fun hello(): String {
        return "hello"
    }

    fun bye(): String {
        return "bye bye"
    }
}
