package ai.qompass.ahoy

import akka.actor.ActorSystem
import akka.http.scaladsl.Http
import akka.stream.Materializer
import scala.concurrent.ExecutionContextExecutor
import scala.util.{Failure, Success}
import ai.qompass.ahoy.routes.UserRoutes

object Main extends App {
  implicit val system: ActorSystem = ActorSystem("AhoySystem")
  implicit val executionContext: ExecutionContextExecutor = system.dispatcher
  implicit val materializer: Materializer = Materializer(system)

  val bindingFuture = Http().newServerAt("0.0.0.0", 9000).bind(UserRoutes.route)

  bindingFuture.onComplete {
    case Success(binding) =>
      val address = binding.localAddress
      println(s"🚀 Ahoy is up at http://${address.getHostName}:${address.getPort}/")
    case Failure(exception) =>
      println(s"❌ Failed to start server: ${exception.getMessage}")
      system.terminate()
  }
}

