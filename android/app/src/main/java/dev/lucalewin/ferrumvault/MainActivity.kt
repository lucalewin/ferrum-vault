package dev.lucalewin.ferrumvault

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import dev.lucalewin.ferrumvault.ui.theme.FerrumVaultTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            FerrumVaultTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    PasswordView(emptyArray(), modifier = Modifier.padding(innerPadding))
                }
            }
        }

        loadAndDisplayServices()
    }
}

private fun loadAndDisplayServices() {

}

@Composable
fun PasswordView(passwords: Array<String>, modifier: Modifier = Modifier) {
    Column {
        passwords.forEach { password ->
            PasswordRow(password, modifier)
        }
    }
}

@Preview(showBackground = true)
@Composable
fun PasswordViewPreview() {
    FerrumVaultTheme {
        PasswordView(emptyArray())
    }
}

@Composable
fun PasswordRow(password: String, modifier: Modifier = Modifier) {
    Text(
        text = password,
        modifier = modifier
    )
}

fun getPasswordsFromApi(): Array<String> {
    return arrayOf("hello", "world");
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    FerrumVaultTheme {
        Greeting("Android")
    }
}
