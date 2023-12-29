package org.demo

import kotlinx.serialization.Serializable

@Serializable
class Extension {
    var id: String? = null
    var name: String? = null
    var shortName: String? = null
    var keywords: List<String>? = null
}
