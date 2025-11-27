package ai.services

import org.mindrot.jbcrypt.BCrypt

object UserService {
  private var users = scala.collection.mutable.Map[String, String]() // email -> hashedPassword

  def register(email: String, password: String): Either[String, String] = {
    if (users.contains(email)) Left("Email already registered")
    else {
      val hashed = BCrypt.hashpw(password, BCrypt.gensalt())
      users(email) = hashed
      Right("User registered")
    }
  }

  def login(email: String, password: String): Boolean = {
    users.get(email).exists(hashed => BCrypt.checkpw(password, hashed))
  }
}

