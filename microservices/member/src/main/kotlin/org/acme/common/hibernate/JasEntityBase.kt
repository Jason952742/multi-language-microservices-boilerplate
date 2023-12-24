package org.acme.common.hibernate

import com.fasterxml.jackson.annotation.JsonIgnore
import jakarta.persistence.Column
import jakarta.persistence.MappedSuperclass
import jakarta.persistence.Version
import org.acme.common.model.SystemDefault
import org.acme.common.resource.JasIdentity
import org.hibernate.annotations.CreationTimestamp
import org.hibernate.annotations.UpdateTimestamp
import java.time.LocalDateTime
import java.util.*

@MappedSuperclass
abstract class JasEntityBase {

    abstract var id: UUID?
    abstract var name: String
    abstract var userId: UUID

    @Column(updatable = false)
    @JsonIgnore
    var orgId: UUID = JasIdentity.current?.orgId ?: SystemDefault.ID

    @Column(updatable = false)
    @JsonIgnore
    var creatorId: UUID = JasIdentity.current?.id ?: SystemDefault.ID

    @Column(updatable = false)
    var creatorName: String = JasIdentity.current?.name ?: SystemDefault.NAME

    @JsonIgnore
    var lastModifierId: UUID = JasIdentity.current?.id ?: SystemDefault.ID
    var lastModifierName: String = JasIdentity.current?.name ?: SystemDefault.NAME

    @CreationTimestamp
    @Column(updatable = false)
    var created: LocalDateTime? = null

    @UpdateTimestamp
    var updated: LocalDateTime? = null

    var hide: Boolean = false
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
