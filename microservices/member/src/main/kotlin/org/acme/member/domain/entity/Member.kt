package org.acme.member.domain.entity

import com.fasterxml.jackson.annotation.JsonIgnore
import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import jakarta.persistence.*
import org.acme.common.base.Party
import org.acme.common.hibernate.JasEntityBase
import org.acme.common.model.Gender
import java.time.LocalDate
import java.util.*

@Entity
@Cacheable
@JsonIgnoreProperties(ignoreUnknown = true, value = ["hibernateLazyInitializer", "handler", "fieldHandler"])
class Member(
    @Id
    @GeneratedValue
    override var id: UUID? = null,

    override var name: String,

    var nickname: String,

    @JsonIgnore
    var loginCreds: String,

    @Enumerated(value = EnumType.STRING)
    var gender : Gender? = null,

    var birth: LocalDate? = null,
    var gravatar: String = "",
    var id_card: String? = null,
    var level: Int = 0,
    var remarks: String = "",
    var hide: Boolean = false,

    @OneToMany(mappedBy = "user", cascade = [CascadeType.ALL], orphanRemoval = true, fetch = FetchType.LAZY)
    var loginPasses: MutableSet<LoginPasses> = mutableSetOf(),

    @OneToOne(cascade = [CascadeType.ALL], orphanRemoval = true)
    @JsonIgnore
    var passwordInfo: PasswordInfo

) : JasEntityBase(), Party
