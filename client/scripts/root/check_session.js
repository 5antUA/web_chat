export async function checkSession() {
    let jwt = localStorage.getItem("jwt");

    if (!jwt) {
        window.location.href = "auth.html";
        return;
    }

    try {
        let res = await fetch("/check_session", {
            method: "GET",
            headers: {
                Authorization: `Bearer ${jwt}`,
            },
        });

        console.log(res.status);
        if (!res.status === 200) {
            localStorage.removeItem("jwt");
            window.location.href = "auth.html";
            return;
        }

        window.location.href = "chat.html";
    } catch (e) {
        console.error(e);
        localStorage.removeItem("jwt");
        window.location.href = "auth.html";
    }
}

checkSession();
