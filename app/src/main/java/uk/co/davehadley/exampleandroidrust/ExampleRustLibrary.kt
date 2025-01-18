package uk.co.davehadley.exampleandroidrust

object ExampleRustLibrary {

     init {
          System.loadLibrary("example_rust_library")
     }

     external fun helloFromRust(name: String): String
}