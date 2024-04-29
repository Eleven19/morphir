import mill._, scalalib._, scalajslib._, scalanativelib._, publish._
import mill.scalajslib.api.{JsEnvConfig, ModuleKind}
import $ivy.`io.eleven19.mill::mill-crossbuild::0.1.0`
import io.eleven19.mill.crossbuild._

object morphir extends CrossPlatform {
  object cli extends CrossPlatform {
    def millSourcePath = os.pwd / "app" / "scalajs-cli"
    trait Shared extends ScalaProject with PlatformScalaModule {
      def ivyDeps =
        Agg(
          Deps.com.github.alexarchambault.`case-app`,
          Deps.com.lihaoyi.mainargs,
          Deps.io.github.cquiroz.`scala-java-time`,
          Deps.io.github.cquiroz.`scala-java-time-tzdb`,
          Deps.com.lihaoyi.sourcecode,
          Deps.dev.zio.`izumi-reflect`,
          Deps.io.github.kitlangton.neotype
        )
    }
    object jvm extends Shared {
      object test extends ScalaTests with TestModule.ZioTest {
        def ivyDeps = Agg(
          Deps.dev.zio.`zio-test`,
          Deps.dev.zio.`zio-test-sbt`
        )
      }
    }
    object js extends Shared with ScalaJSProject {
      override def jsEnvConfig: Target[JsEnvConfig] = T {
        JsEnvConfig.NodeJs()
      }
      override def mainClass: T[Option[String]] = T {
        Some("morphir.Main")
      }
      override def moduleKind = T { ModuleKind.CommonJSModule }
      object test extends ScalaJSTests with TestModule.ZioTest {
        def ivyDeps = Agg(
          Deps.dev.zio.`zio-test`,
          Deps.dev.zio.`zio-test-sbt`
        )
      }
    }
  }

  trait Shared extends ScalaProject with PlatformScalaModule
  object jvm extends Shared
  object js extends Shared with ScalaJSProject
}

trait ScalaProject extends ScalaModule {
  def scalaVersion = Versions.scala
}

trait ScalaJSProject extends ScalaJSModule {
  def scalaJSVersion = Versions.scalaJS
}

//---------------------------------------------------------------------
// Dependencies and Versions
//---------------------------------------------------------------------
object Deps {
  case object com {
    case object github {
      case object alexarchambault {
        val `case-app` =
          ivy"com.github.alexarchambault::case-app::${Versions.`case-app`}"
      }
      case object `j-mie6` {
        val parsley = ivy"com.github.j-mie6::parsley::${Versions.parsley}"
      }
    }

    case object lihaoyi {
      val mainargs = ivy"com.lihaoyi::mainargs::${Versions.mainargs}"
      val sourcecode = ivy"com.lihaoyi::sourcecode::${Versions.sourcecode}"
    }
  }
  case object dev {
    case object zio {
      val `izumi-reflect` =
        ivy"dev.zio::izumi-reflect::${Versions.`izumi-reflect`}"
      val zio: Dep = ivy"dev.zio::zio::${Versions.zio}"
      val `zio-cli` = ivy"dev.zio::zio-cli::${Versions.`zio-cli`}"
      val `zio-config` = config()
      val `zio-interop-cats` =
        ivy"dev.zio::zio-interop-cats::${Versions.`zio-interop-cats`}"
      val `zio-json`: Dep = ivy"dev.zio::zio-json::${Versions.`zio-json`}"
      val `zio-json-golden` =
        ivy"dev.zio::zio-json-golden::${Versions.`zio-json`}"
      val `zio-parser` = ivy"dev.zio::zio-parser::${Versions.`zio-parser`}"
      val `zio-nio` = ivy"dev.zio::zio-nio::${Versions.`zio-nio`}"
      val `zio-prelude` = prelude()
      val `zio-prelude-macros` = prelude.macros
      val `zio-process` = ivy"dev.zio::zio-process::${Versions.`zio-process`}"
      val `zio-streams` = ivy"dev.zio::zio-streams::${Versions.zio}"
      val `zio-test` = ivy"dev.zio::zio-test::${Versions.zio}"
      val `zio-test-magnolia` = ivy"dev.zio::zio-test-magnolia::${Versions.zio}"
      val `zio-test-sbt` = ivy"dev.zio::zio-test-sbt::${Versions.zio}"

      object config {
        def apply(): Dep = ivy"dev.zio::zio-config::${Versions.`zio-config`}"
        val magnolia =
          ivy"dev.zio::zio-config-magnolia::${Versions.`zio-config`}"
        val refined = ivy"dev.zio::zio-config-refined::${Versions.`zio-config`}"
        val typesafe =
          ivy"dev.zio::zio-config-typesafe::${Versions.`zio-config`}"
      }

      case object prelude {
        def apply(): Dep = ivy"dev.zio::zio-prelude::${Versions.`zio-prelude`}"
        val macros = ivy"dev.zio::zio-prelude-macros::${Versions.`zio-prelude`}"
      }

      case object schema {
        val `avro` = ivy"dev.zio::zio-schema-avro::${Versions.`zio-schema`}"
        val `bson` = ivy"dev.zio::zio-schema-bson::${Versions.`zio-schema`}"
        val `core` = ivy"dev.zio::zio-schema-core::${Versions.`zio-schema`}"
        val `derivation` =
          ivy"dev.zio::zio-schema-derivation::${Versions.`zio-schema`}"
        val `json` = ivy"dev.zio::zio-schema-json::${Versions.`zio-schema`}"
        val `msg-pack` =
          ivy"dev.zio::zio-schema-msg-pack::${Versions.`zio-schema`}"
      }
    }
  }

  case object io {
    case object getkyo {
      val `kyo-core` = ivy"io.getkyo::kyo-core::${Versions.kyo}"
      val `kyo-direct` = ivy"io.getkyo::kyo-direct::${Versions.kyo}"
      val `kyo-sttp` = ivy"io.getkyo::kyo-sttp::${Versions.kyo}"

    }
    case object github {
      case object cquiroz {
        val `scala-java-time` =
          ivy"io.github.cquiroz::scala-java-time::${Versions.`scala-java-time`}"
        val `scala-java-time-tzdb` =
          ivy"io.github.cquiroz::scala-java-time-tzdb::${Versions.`scala-java-time`}"
      }
      case object iltotore {
        val iron = ivy"io.github.iltotore::iron::${Versions.iron}"
      }
      case object kitlangton {
        val `neotype` = ivy"io.github.kitlangton::neotype::${Versions.neotype}"

      }
    }
  }

  case object org {
    case object wvlet {
      case object airframe {
        val `airframe-surface` =
          ivy"org.wvlet.airframe::airframe-surface:${Versions.airframe}"
      }
    }
  }
}

object Versions {
  val airframe = "24.4.0"
  val `case-app` = "2.1.0-M26"
  val iron = "2.5.0"
  val `izumi-reflect` = "2.3.8"
  val kyo = "0.9.2"
  val mainargs = "0.7.0"
  val neotype = "0.2.5"
  val parsley = "4.5.1"
  val scala = "3.3.3"
  val `scala-java-time` = "2.5.0"
  val scalaJS = "1.16.0"
  val scalaNative = "0.5.1"
  val sourcecode = "0.4.1"
  val zio = "2.0.21"
  val `zio-cli` = "0.5.0"
  val `zio-config` = "4.0.1"
  val `zio-interop-cats` = "23.1.0.1"
  val `zio-json` = "0.6.2"
  val `zio-nio` = "2.0.2"
  val `zio-parser` = "0.1.9"
  val `zio-prelude` = "1.0.0-RC23"
  val `zio-process` = "0.7.2"
  val `zio-schema` = "0.4.12"
}
