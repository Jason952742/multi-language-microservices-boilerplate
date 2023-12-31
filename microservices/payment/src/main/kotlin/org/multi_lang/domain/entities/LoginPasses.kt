package org.multi_lang.domain.entity

import com.fasterxml.jackson.annotation.JsonIgnore
import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import jakarta.persistence.*
import org.shared.common.hibernate.JasEntityBase
import org.multi_lang.domain.entity.enums.IdentityMold
import java.time.LocalDateTime
import java.util.*


/**
 * LoginPasses
 *
 * @param id
 * @param user
 * @param loginCreds Login information (joint identity id for binding identity (mobile phone number for now))
 * @param mold identity mold
 * @param identifier login identity（email、phone、username...）
 * @param expired
 */
@Entity
@Cacheable
@JsonIgnoreProperties(ignoreUnknown = true, value = ["hibernateLazyInitializer", "handler", "fieldHandler"])
class LoginPasses(
    @Id
    @GeneratedValue
    override var id: UUID? = null,
    @Column(updatable = false, unique = true)
    override var userId: UUID,

    override var name: String,

    var loginCreds: String,

    @Enumerated(value = EnumType.STRING)
    var mold: IdentityMold,

    var identifier: String,
    var expired: LocalDateTime,

    @ManyToOne(fetch = FetchType.EAGER)
    @JsonIgnore
    var user: org.multi_lang.domain.entity.Member

) : JasEntityBase() {

    fun active() = LocalDateTime.now().isBefore(expired)
}

