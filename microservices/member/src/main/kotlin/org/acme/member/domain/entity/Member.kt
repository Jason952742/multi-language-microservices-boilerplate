package org.acme.member.domain.entity

import com.fasterxml.jackson.annotation.JsonIgnore
import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import jakarta.persistence.*
import org.acme.common.base.Party
import org.acme.common.hibernate.JasEntityBase
import org.acme.common.model.MemberType
import org.acme.member.domain.enums.MemberStatus
import java.time.LocalDateTime
import java.util.*

@Entity
@Cacheable
@JsonIgnoreProperties(ignoreUnknown = true, value = ["hibernateLazyInitializer", "handler", "fieldHandler"])
class Member(
    @Id
    @GeneratedValue
    override var id: UUID? = null,
    @Column(updatable = false, unique = true)
    override var name: String,
    @Column(updatable = false, unique = true)
    override var userId: UUID,

    var nickname: String,

    @Enumerated(value = EnumType.STRING)
    var status: MemberStatus = MemberStatus.Created,

    @Enumerated(value = EnumType.STRING)
    var memberType: MemberType = MemberType.Wood,

    @JsonIgnore
    @Column(length = 40, unique = true)
    var loginCreds: String,

    var point: Long = 0,
    var creditScore: Double = 0.0,
    var level: Int = 0,
    var expiredAt: LocalDateTime = LocalDateTime.now(),
    var referrerCode: String,

    var description: String = "",
    var lastLoginAt: LocalDateTime? = null,

    @OneToMany(mappedBy = "user", cascade = [CascadeType.ALL], orphanRemoval = true, fetch = FetchType.LAZY)
    var loginPasses: MutableSet<LoginPasses> = mutableSetOf(),

    @OneToOne(cascade = [CascadeType.ALL], orphanRemoval = true)
    @JsonIgnore
    var passwordInfo: PasswordInfo

) : JasEntityBase(), Party
