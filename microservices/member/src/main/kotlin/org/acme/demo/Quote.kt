package org.acme.demo

import io.quarkus.runtime.annotations.RegisterForReflection


@RegisterForReflection
data class Quote (
    var id: String? = null,
    var price: Int = 0
) {

    override fun toString(): String {
        return "Quote{" +
            "id='" + id + '\'' +
            ", price=" + price +
            '}'
    }
}
