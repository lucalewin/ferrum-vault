package dev.lucalewin.ferrumvault.ui

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.wrapContentHeight
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Menu
import androidx.compose.material3.Card
import androidx.compose.material3.Icon
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.dimensionResource
import androidx.compose.ui.unit.dp
import dev.lucalewin.ferrumvault.R

@Composable
fun PasswordView() {
    val mediumPadding = dimensionResource(R.dimen.padding_medium)

    Surface(modifier = Modifier.fillMaxSize()) {
        Column(
            modifier = Modifier
                .padding(mediumPadding)
                .fillMaxWidth()
        ) {
//            item {
//                PasswordRow(password = "First item")
//            }
            PasswordRow("abc")
            PasswordRow("def")
            PasswordRow("ghi")
            PasswordRow("ghi")

        }
        Text("---------")
        LazyColumn {
            item {
                PasswordRow("ghi")
            }
        }
    }
}

@Composable
fun PasswordRow(password: String) {
    Card(
        modifier = Modifier.padding(2.dp)
    ) {
        Row(
            modifier = Modifier.padding(8.dp).fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = Icons.Filled.Menu,
                contentDescription = "Localized description"
            )
            Spacer(modifier = Modifier.width(8.dp))
            Text("name")
            Spacer(modifier = Modifier.width(8.dp))
            Text("username")
            Spacer(modifier = Modifier.width(8.dp))
            Text("password: $password")
        }
    }
}
