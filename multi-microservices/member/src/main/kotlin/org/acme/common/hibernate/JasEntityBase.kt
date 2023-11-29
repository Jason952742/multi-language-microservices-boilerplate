package org.acme.common.hibernate

import com.fasterxml.jackson.annotation.JsonIgnore
import jakarta.persistence.Column
import jakarta.persistence.MappedSuperclass
import jakarta.persistence.Version
import org.acme.common.resource.JasIdentity
import org.hibernate.annotations.CreationTimestamp
import org.hibernate.annotations.UpdateTimestamp
import java.time.LocalDateTime
import java.util.*

@MappedSuperclass
abstract class JasEntityBase {

    abstract var id: UUID?

    abstract var name: String

    @Column(updatable = false)
    @JsonIgnore
    var org_id: UUID = JasIdentity.current?.org_id ?: UUID.fromString("88888888-8888-8888-8888-888888888888")

    @Column(updatable = false)
    @JsonIgnore
    var creator_id: UUID = JasIdentity.current?.id ?: UUID.fromString("88888888-8888-8888-8888-888888888888")

    @Column(updatable = false)
    var creator_name: String = JasIdentity.current?.name ?: "system"

    @JsonIgnore
    var last_modifier_id: UUID = JasIdentity.current?.id ?: UUID.fromString("88888888-8888-8888-8888-888888888888")
    var last_modifier_name: String = JasIdentity.current?.name ?: "system"

    @CreationTimestamp
    @Column(updatable = false)
    var created: LocalDateTime? = null

    @UpdateTimestamp
    var updated: LocalDateTime? = null

    var enabled: Boolean = true

    @Version
    var version: Long = 0

    @JsonIgnore
    var deletion: Boolean = false

    companion object {
        private const val serialVersionUID = -5554308939380869754L
    }

    override fun equals(other: Any?): Boolean {
        other ?: return false

        if (this === other) return true

        if (javaClass !== other.javaClass) return false

        other as JasEntityBase

        return if (null == this.id) false else this.id == other.id
    }

    override fun hashCode(): Int {
        val result = id.hashCode()
        return Objects.hash(31 * result)
    }

    override fun toString() = "Entity of type ${this.javaClass.name} with id: $id $name"

}