package ai.routes

import akka.http.scaladsl.server.Directives._
import akka.http.scaladsl.model.StatusCodes
import akka.http.scaladsl.server.Route
import de.heikoseeberger.akkahttpcirce.FailFastCirceSupport._
import io.circe.generic.auto._
import ai.services.UserService
import ai.models.{User, RegisterRequest, LoginRequest, AuthResponse}

object UserRoutes {
  val route: Route =
    pathPrefix("api" / "users") {
      concat(
        path("register") {
          post {
            entity(as[RegisterRequest]) { req =>
              onSuccess(UserService.register(req.email, req.password)) {
                case Right(user) => complete(StatusCodes.Created -> AuthResponse(s"User ${user.email} registered"))
                case Left(error) => complete(StatusCodes.BadRequest -> AuthResponse(error))
              }
            }
          }
        },
        path("login") {
          post {
            entity(as[LoginRequest]) { req =>
              onSuccess(UserService.login(req.email, req.password)) {
                case Right(token) => complete(AuthResponse(s"Token: $token"))
                case Left(error)  => complete(StatusCodes.Unauthorized -> AuthResponse(error))
              }
            }
          }
        },
        path("me") {
          get {
            optionalHeaderValueByName("X-Auth-Request-Email") {
              case Some(email) =>
                onSuccess(UserService.lookup(email)) {
                  case Some(user) => complete(user)
                  case None => complete(StatusCodes.NotFound -> AuthResponse("User not found"))
                }
              case None => complete(StatusCodes.Unauthorized -> AuthResponse("Missing auth headers"))
            }
          }
        }
      )
    }
}
