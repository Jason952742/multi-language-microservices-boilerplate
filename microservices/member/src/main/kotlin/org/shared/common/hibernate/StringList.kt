package org.shared.common.hibernate

import jakarta.persistence.AttributeConverter
import java.util.stream.Collectors

data class StringList(val values: MutableList<String>) : java.io.Serializable {

    @jakarta.persistence.Converter(autoApply = true)
    class Converter : AttributeConverter<StringList, String> {
        override fun convertToDatabaseColumn(attribute: StringList): String {
            return attribute.values.stream().collect(Collectors.joining(","))
        }

        override fun convertToEntityAttribute(dbData: String): StringList {
            return StringList(dbData.split(",").toMutableList())
        }

    }
}
