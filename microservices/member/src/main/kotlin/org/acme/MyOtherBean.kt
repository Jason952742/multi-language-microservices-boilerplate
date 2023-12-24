package org.acme

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
