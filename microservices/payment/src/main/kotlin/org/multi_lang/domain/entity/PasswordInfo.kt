package org.multi_lang.domain.entity

import jakarta.persistence.*
import org.shared.common.hibernate.JasEntityBase
import org.shared.utils.EncryptionUtils
import org.shared.utils.EncryptionUtils.encrypt
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
