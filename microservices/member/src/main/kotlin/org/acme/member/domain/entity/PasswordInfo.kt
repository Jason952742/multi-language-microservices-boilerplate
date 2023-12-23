package org.acme.member.domain.entity

import jakarta.persistence.*
import org.acme.common.hibernate.JasEntityBase
import org.acme.utils.EncryptionUtils
import org.acme.utils.EncryptionUtils.encrypt
import java.util.*

@Entity
@Cacheable
class PasswordInfo(
    @Id
    @GeneratedValue
    override var id: UUID? = null,
    @Column(updatable = false, unique = true)
    override var userId: UUID,

    override var name: String,

    var loginCreds: String,
    var hasher: String = this.hashCode().toString(),
    var password: String,
    var salt: String = EncryptionUtils.salt
) : JasEntityBase() {

    fun updateNewPassword(newPassword: String) = this.apply {
        this.password = newPassword.encrypt()
    }
}
