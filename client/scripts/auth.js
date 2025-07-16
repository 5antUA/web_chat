import { concatPath } from "./paths.js";

let mainText = document.getElementById("mainText");
let usernameForm = document.getElementById("usernameInput");
let passwordForm = document.getElementById("passwordInput");

let authBtn = document.getElementById("authBtn");
let registerBtn = document.getElementById("registerBtn");

authBtn.addEventListener("click", () => {
    let username = usernameForm.value;

    fetch(concatPath(`/users/${username}`))
        .then((responce) => {
            if (responce.ok) {
                // go to profile...
            }
        })
        .catch((error) => {
            console.error(`Oops... you catch auth error: ${error}`);
        });
});

registerBtn.addEventListener("click", () => {
    let username = usernameForm.value;
    let password_hash = passwordForm.value;

    fetch(concatPath("/users/register"), {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            username,
            password_hash,
            role_name: "User",
        }),
    })
        .then(async (res) => {
            if (!res.ok) {
                const errorText = await res.text();
                throw { status: res.status, message: errorText };
            }
            return res.json();
        })
        .then((data) => {
            mainText.textContent = `Користувач ${data.username} був успішно зареєстрований!`;
            usernameForm.value = "";
            passwordForm.value = "";
        })
        .catch((error) => {
            switch (error.status) {
                case 400:
                    mainText.textContent = "Дані заповнені некоректно!";
                    break;
                case 409:
                    mainText.textContent =
                        "Користувач з таким іменем вже існує!";
                    break;
                case 500:
                    mainText.textContent = "Сервер помер. Спробуй пізніше.";
                    break;
                default:
                    mainText.textContent = `Невідома помилка: ${
                        error.message || error
                    }`;
            }
            console.error("Register error:", error);
        });
});
