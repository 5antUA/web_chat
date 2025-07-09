import { concatPath } from "./paths.js";

// LOGIN ELEMENTS
let nicknameField = document.getElementById("nickname");
let loginBtn = document.getElementById("loginBtn");
let nicknameForm = document.getElementById("userDisplay");

// CHAT ELEMENTS
let sendBtn = document.getElementById("sendBtn");
let messageField = document.getElementById("messageInput");
let textArea = document.getElementById("chatOutput");

//LOGIC

sendBtn.addEventListener("click", function () {
    let message = messageField.value;

    if (isNullOrEmpty(message)) {
        return;
    }

    messageField.value = "";
    textArea.innerHTML += `${message}<br>`;
});

loginBtn.addEventListener("click", function () {
    let name = nicknameField.value;

    fetch(concatPath("/json"), {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ name: name, id: 1 }),
    })
        .then((data) => data.text())
        .then((data) => (nicknameForm.textContent = `${data}`));
});

function isNullOrEmpty(string) {
    return !string;
}
