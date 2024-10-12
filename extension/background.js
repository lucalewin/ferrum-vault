// chrome.runtime.onInstalled.addListener(() => {
//     chrome.storage.sync.set({ passwords: [] });
// })

chrome.runtime.onStartup.addListener(function() {
    console.log('Loaded Ferrum Vault');
    // TODO: request list of passwords (not the actual passwords) from the api
})

chrome.webNavigation.onCompleted.addListener(({ tabId, frameId }) => {
    if (frameId !== 0) return;

    chrome.scripting.executeScript({
        target: { tabId },
        function: newPageLoad,
    })
})

const newPageLoad = async () => {
    console.log("[Ferrum Vault] :: INFO: loaded");
    let disabled = false;
    let selected = null;

    const url = "lucalewin.dev"
    const passwords = [
        {
            id: 1,
            username: "test@example.com",
            password: "1234"
        },
        {
            id: 2,
            username: "contact@lucalewin.dev",
            password: "abcdefg"
        }
    ]

    // create modal element
    const modal = document.createElement("div");
    let content = `
    <div class="modal-content">
        <span class="close">&times;</span>
        <h2 style="color: darkorange">Ferrum Vault</h2>
        <small>Signing in to ${url}</small>
    `;
    for (let i = 0; i < passwords.length; i++) {
        content += `
        <div class="modal-card" data-ferrum-vault="${passwords[i].id}">
            <!-- add icon -->
            <p>${passwords[i].username}</p>
            <p>***********</p>
        </div>
        `;
    }
    content += "</div>"
    modal.innerHTML = content;

    modal.classList.add('modal');
    modal.style.display = "none";
    document.body.appendChild(modal);

    const injectedStyle = `
        .modal {
            display: none;
            position: absolute;
            top: 0;
            left: 0;
            width: 100vw;
            height: 100svh;
            background-color: rgba(0, 0, 0, 0.4);
            z-index: 1;
            font-size: 15px;
        }

        .modal-content {
            color: #fff;
            background-color: #222;
            margin: 0 auto;
            position: absolute;
            top: 32px;
            right: 32px;
            /* right: 50%;
            bottom: 50%;
            transform: translate(50%, -50%); */
            padding: 20px;
            border: 1px solid #666;
            border-radius: 12px;
            width: 256px;
        }
        .modal-content h2 {
            font-size: initial;
        }
        .modal-card {
            padding: 8px;
            margin: 4px 0;
            background-color: #333;
            border-radius: 4px;
            cursor: pointer;
        }
        .modal-card:hover {
            background-color: #444;
        }
        .close {
            color: #aaa;
            float: right;
            font-size: 28px;
            font-weight: bold;
        }
        .close:hover,
        .close:focus {
            color: red;
            text-decoration: none;
            cursor: pointer;
        }
    `;

    const test = document.createElement("style")
    test.textContent = injectedStyle;
    document.head.appendChild(test)

    console.log("added modal");

    var span = document.getElementsByClassName("close")[0];
    span.onclick = function() {
        modal.style.display = "none";
        disabled = true;
    }
    window.onclick = function(event) {
        if (event.target == modal) {
            modal.style.display = "none";
            disabled = true;
        }
    }

    // add listeners to modal-cards
    const cards = document.getElementsByClassName("modal-card");
    for (let i = 0; i < cards.length; i++) {
        cards[i].addEventListener("click", function() {
            if (selected == null || selected == undefined) return;

            let id = cards[i].getAttribute("data-ferrum-vault");
            let password_entry = undefined;
            for (let j = 0; j < passwords.length; j++) {
                if (passwords[j].id == id) {
                    password_entry = passwords[j];
                    break;
                }
            }
            if (password_entry == undefined) return;

            let form = selected.closest("form");
            let inputs = form.getElementsByTagName("input");

            for (let j = 0; j < inputs.length; j++) {
                let input = inputs[j];
        
                if (input.type === "password") {
                    input.value = password_entry.password;
                } else if (input.type === "email" || input.type === "username") {
                    input.value = password_entry.username;
                }
            }

            form.submit()
        });
    }


    // add listeners to input elements
    let forms = document.getElementsByTagName("form");
    for (let i = 0; i < forms.length; i++) {
        let form = forms[i];
        let inputs = form.getElementsByTagName("input");

        let containsPasswordInput = false;
        for (let j = 0; j < inputs.length; j++) {
            let input = inputs[j];
    
            if (input.type === "password") {
                containsPasswordInput = true;
            }
        }

        if (!containsPasswordInput) {
            continue;
        }

        // else check if username/email and password can be autofilled
        console.log("Form could be autofilled!")

        for (let j = 0; j < inputs.length; j++) {
            inputs[j].addEventListener('focus', function() {
                if (disabled) return;

                selected = inputs[j]
                console.log("focus")

                // const rect = inputs[j].getBoundingClientRect();
                // const inputHeight = rect.height;

                // // Position the modal below the input
                // modal.style.top = `${rect.bottom + inputHeight}px`;
                // modal.style.left = `${rect.left}px`;

                modal.style.display = "block";
            });
        }
    }
}
