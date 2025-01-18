package uk.co.davehadley.exampleandroidrust

import androidx.test.ext.junit.runners.AndroidJUnit4

import org.junit.Test
import org.junit.runner.RunWith

import org.junit.Assert.*

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class TestExampleRustLibrary {

    @Test
    fun testHelloFromRust() {
        val actual = ExampleRustLibrary.helloFromRust("Dave")
        val expected = "Hello Dave from Rust!"
        assertEquals(expected, actual)
    }
}