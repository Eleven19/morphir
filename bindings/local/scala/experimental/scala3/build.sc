import mill._, scalalib._, scalajslib._, scalanativelib._, publish._
import $ivy.`io.eleven19.mill::mill-crossbuild::0.1.0`
import io.eleven19.mill.crossbuild._




object morphir extends CrossPlatform {
    object core extends CrossPlatform {
        trait Shared extends  ScalaProject with PlatformScalaModule
        object jvm extends Shared 
        object js extends Shared with ScalaJSProject        
    }    

    trait Shared extends ScalaProject with PlatformScalaModule
    object jvm extends Shared 
    object js extends Shared with ScalaJSProject    
}

trait ScalaProject extends ScalaModule {
    def scalaVersion = Deps.scalaVersion
}

trait ScalaJSProject extends ScalaJSModule {
    def scalaJSVersion = Deps.scalaJSVersion
}

object Deps {
    val scalaVersion = "3.3.3"
    val scalaJSVersion = "1.16.0"
}