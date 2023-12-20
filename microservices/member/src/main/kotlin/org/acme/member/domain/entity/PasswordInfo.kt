package org.acme.member.domain.entity

import jakarta.persistence.Cacheable
import jakarta.persistence.Entity
import jakarta.persistence.GeneratedValue
import jakarta.persistence.Id
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
