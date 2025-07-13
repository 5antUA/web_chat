import { concatPath } from "./paths.js";

// LOGIN ELEMENTS
let userField = document.getElementById("nickname");
let loginBtn = document.getElementById("loginBtn");
let registerBtn = document.getElementById("registerBtn");
let userInfo = document.getElementById("userDisplay");

// CHAT ELEMENTS
let sendBtn = document.getElementById("sendBtn");
let msgField = document.getElementById("messageInput");
let chatArea = document.getElementById("chatOutput");

// SEND MSG
sendBtn.addEventListener("click", function () {
    let message = msgField.value;

    if (isNullOrEmpty(message)) {
        return;
    }

    msgField.value = "";
    chatArea.innerHTML += `${message}<br>`;
});

// LOGIN
loginBtn.addEventListener("click", function () {
    let name = userField.value;

    if (isNullOrEmpty(name)) {
        return;
    }

    fetch(concatPath(`/users/${name}`))
        .then((data) => data.json())
        .then((data) => (userInfo.textContent = `Вітаємо, ${data.username}!`))
        .catch((_) => {
            userInfo.textContent = "Вибачте, такого користувача не знайдено.";
        });
});

// REGISTER
registerBtn.addEventListener("click", function () {
    let username = userField.value;

    if (isNullOrEmpty(username)) {
        console.log("is null");
        return;
    }

    fetch(concatPath("/users/register"), {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            username, // unique username
            email: "uniqueLogin", // unique login
            password_hash: "sexpass",
        }),
    })
        .then((data) => data.json())
        .then(
            (data) =>
                (userInfo.textContent = `Успішна реєстрація, ${data.username}!`)
        )
        .catch((_) => {
            userInfo.textContent = "Користувач з таким іменем вже існує!";
        });
});

function isNullOrEmpty(string) {
    return !string || string.length == 0;
}
