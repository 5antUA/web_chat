const API_URL = "http://localhost:8080/api";

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
    // fetch(`${API_URL}/hello`)
    //     .then((data) => data.json())
    //     .then((data) => {
    //         nicknameForm.textContent = `Вітаємо, ${data}!`;
    //     });

    let name = nicknameField.value;

    fetch(`${API_URL}/json`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ name }),
    })
        .then((data) => data.text())
        .then((data) => (nicknameForm.textContent = `${data}`));
});

function isNullOrEmpty(string) {
    return !string;
}
